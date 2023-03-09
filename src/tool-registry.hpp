#pragma once

#include <vector>
#include <string>
#include <memory>

namespace v {
    struct Tool {
        virtual std::string& getLabel() = 0;
        virtual void render() = 0;
    };

    struct ToolRegistry {
        std::vector<std::shared_ptr<Tool>> tools;
        int selectedIdx = 0;
    };
}
