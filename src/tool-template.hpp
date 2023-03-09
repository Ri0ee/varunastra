#pragma once

#include "hello_imgui/hello_imgui.h"
#include "tool-registry.hpp"
#include <string>

namespace v {
    struct ToolTemplate : Tool {
        std::string label = "Template tool";

        std::string& getLabel() { 
            return label;
        }

        void render() {
            ImGui::Text(label.c_str());
        }
    };
}