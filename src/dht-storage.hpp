#pragma once

#define _WIN32_WINNT 0x0601

#include <string>
#include <libtorrent/session.hpp>
#include <libtorrent/alert_types.hpp>
#include <libtorrent/bencode.hpp> // for bencode()
#include <libtorrent/kademlia/item.hpp> // for sign_mutable_item
#include <libtorrent/kademlia/ed25519.hpp>
#include <libtorrent/span.hpp>
#include <libtorrent/session_params.hpp>

#include <functional>
#include <cstdio> // for snprintf
#include <cinttypes> // for PRId64 et.al.
#include <cstdlib>
#include <fstream>

#include "hello_imgui/hello_imgui.h"

using namespace lt;
using namespace lt::dht;
using namespace std::placeholders;

namespace v {
    struct DHTStorage {
        DHTStorage() {
            try {
                sp = load_dht_state();
                sp.settings.set_bool(settings_pack::enable_dht, true);
                sp.settings.set_int(settings_pack::alert_mask, 0x7fffffff);
                s = std::make_shared<lt::session>(sp);
                bootstrap(*s);
            }
            catch (std::exception& e) {
                std::cout << e.what() << '\n';
            }
        }

        std::string to_hex(lt::span<char const> key) {
            std::string out;
            for (auto const b : key) {
                char buf[3]{};
                std::snprintf(buf, sizeof(buf), "%02x", static_cast<unsigned char>(b));
                out += static_cast<char const*>(buf);
            }
            return out;
        }

        int generate_key(char const* filename) {
            std::array<char, 32> seed = ed25519_create_seed();

            std::fstream f(filename, std::ios_base::out | std::ios_base::binary);
            f.write(seed.data(), seed.size());
            if (f.fail())
            {
                std::fprintf(stderr, "failed to write key file.\n");
                return 1;
            }

            return 0;
        }

        int hex_to_int(char in) {
            if (in >= '0' && in <= '9') return int(in) - '0';
            if (in >= 'A' && in <= 'F') return int(in) - 'A' + 10;
            if (in >= 'a' && in <= 'f') return int(in) - 'a' + 10;
            return -1;
        }

        bool from_hex(span<char const> in, span<char> out) {
            if (in.size() != out.size() * 2) return false;
            auto o = out.begin();
            for (auto i = in.begin(); i != in.end(); ++i, ++o)
            {
                int const t1 = hex_to_int(*i);
                if (t1 == -1) return false;
                ++i;
                if (i == in.end()) return false;
                int const t2 = hex_to_int(*i);
                if (t2 == -1) return false;
                *o = static_cast<char>((t1 << 4) | (t2 & 0xf));
            }
            return true;
        }

        void bootstrap(lt::session& s) {
            wait_for_alert(s, dht_bootstrap_alert::alert_type);
            HelloImGui::Log(HelloImGui::LogLevel::Info, "Bootstrap done");
        }

        alert* wait_for_alert(lt::session& s, int alert_type) {
            alert* ret = nullptr;
            while (!ret)
            {
                s.wait_for_alert(seconds(5));

                std::vector<alert*> alerts;
                s.pop_alerts(&alerts);
                for (auto const a : alerts) {
                    if (a->type() != alert_type) continue;
                    ret = a;
                }
            }
            std::printf("\r");
            return ret;
        }

        void put_string(entry& e, std::array<char, 64>& sig
            , std::int64_t& seq
            , std::string const& salt
            , std::array<char, 32> const& pk
            , std::array<char, 64> const& sk
            , char const* str)
        {
            using lt::dht::sign_mutable_item;

            e = std::string(str);
            std::vector<char> buf;
            bencode(std::back_inserter(buf), e);
            dht::signature sign;
            ++seq;
            sign = sign_mutable_item(buf, salt, dht::sequence_number(seq)
                , dht::public_key(pk.data())
                , dht::secret_key(sk.data()));
            sig = sign.bytes;
        }

        lt::session_params load_dht_state() {
            std::fstream f(".dht", std::ios_base::in | std::ios_base::binary);
            f.unsetf(std::ios_base::skipws);
            std::printf("load dht state from .dht\n");
            std::vector<char> const state(std::istream_iterator<char>{f}, std::istream_iterator<char>{});

            if (f.bad() || state.empty()) {
                std::fprintf(stderr, "failed to read .dht\n");
                return {};
            }
            return read_session_params(state);
        }

        std::string put(const std::string& str) {
            entry data = str;
            sha1_hash const target = s->dht_put_item(data);
            alert* a = wait_for_alert(*s, dht_put_alert::alert_type);
		    dht_put_alert* pa = alert_cast<dht_put_alert>(a);
            
            return to_hex(target);
        }

        std::string get(const std::string& hash) {
            sha1_hash target;
            from_hex({hash.c_str(), 40}, target);
            s->dht_get_item(target);

            alert* a = wait_for_alert(*s, dht_immutable_item_alert::alert_type);
            dht_immutable_item_alert* item = alert_cast<dht_immutable_item_alert>(a);

            return item->item.string();
        }

        lt::session_params sp;
        std::shared_ptr<lt::session> s = nullptr;
    };
}