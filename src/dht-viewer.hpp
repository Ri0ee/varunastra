#pragma once

#include "hello_imgui/hello_imgui.h"
#include "tool-registry.hpp"
#include "dht-storage.hpp"
#include <string>
#include <memory>
#include "imgui_stdlib.h"

namespace v {
    struct DHTViewer : Tool {
        DHTViewer() {
            getHash.resize(41);
            getText = "";

            putHash = "";
            putText = "";

            dhtStorage = std::make_shared<DHTStorage>();
        }

        std::string label = "DHT Viewer";

        std::string getHash, putHash;
        std::string getText, putText;
        std::shared_ptr<DHTStorage> dhtStorage = nullptr;

        std::string& getLabel() {
            return label;
        }

        void render() {
            ImGui::InputText("Key##get", (char*)getHash.c_str(), getHash.size());
            if (ImGui::Button("Get##get")) {
                getText = dhtStorage->get(getHash);
            }
            ImGui::InputText("Value##get", &getText, ImGuiInputTextFlags_ReadOnly);

            ImGui::Separator();

            ImGui::InputText("Value##put", &putText);
            if (ImGui::Button("Put##put")) {
                HelloImGui::Log(HelloImGui::LogLevel::Info, "size: %d", putText.size());
                HelloImGui::Log(HelloImGui::LogLevel::Info, "value: %s", putText.c_str());
                putHash = dhtStorage->put(putText);
            }
            ImGui::InputText("Key##put", &putHash, ImGuiInputTextFlags_ReadOnly);
        }
    };
}