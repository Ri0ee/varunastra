#pragma once
#include "hello_imgui/hello_imgui.h"
#include "state.hpp"

namespace v {
    struct UI {
        int Run() {
            HelloImGui::RunnerParams runnerParams;

            runnerParams.appWindowParams.windowTitle = "Varunastra";
            runnerParams.appWindowParams.windowGeometry = {800, 600};
            runnerParams.appWindowParams.restorePreviousGeometry = true;
            runnerParams.appWindowParams.resizable = true;
            runnerParams.appWindowParams.borderless = false;
            
            runnerParams.imGuiWindowParams.defaultImGuiWindowType = HelloImGui::DefaultImGuiWindowType::ProvideFullScreenDockSpace;
            runnerParams.imGuiWindowParams.enableViewports = true;
            
            HelloImGui::DockingSplit splitMainBottom;
            splitMainBottom.initialDock = "MainDockSpace";
            splitMainBottom.newDock = "BottomSpace";
            splitMainBottom.direction = ImGuiDir_Down;
            splitMainBottom.ratio = 0.25f;

            HelloImGui::DockingSplit splitMainLeft;
            splitMainLeft.initialDock = "MainDockSpace";
            splitMainLeft.newDock = "LeftSpace";
            splitMainLeft.direction = ImGuiDir_Left;
            splitMainLeft.ratio = 0.25f;

            runnerParams.dockingParams.dockingSplits = { splitMainBottom, splitMainLeft };

            HelloImGui::DockableWindow toolkitWindow;
            toolkitWindow.label = "Toolkit";
            toolkitWindow.dockSpaceName = "LeftSpace";
            toolkitWindow.GuiFunction = [this] { ToolkitWindow(); };
            toolkitWindow.canBeClosed = false;

            HelloImGui::DockableWindow toolWindow;
            toolWindow.label = "Tool";
            toolWindow.dockSpaceName = "MainDockSpace";
            toolWindow.GuiFunction = [this] { ToolWindow(); };
            toolWindow.canBeClosed = false;

            HelloImGui::DockableWindow logWindow;
            logWindow.label = "Log";
            logWindow.dockSpaceName = "BottomSpace";
            logWindow.GuiFunction = [] { HelloImGui::LogGui(); };
            logWindow.canBeClosed = false;

            runnerParams.dockingParams.dockableWindows = { toolkitWindow, logWindow, toolWindow };

            HelloImGui::Log(HelloImGui::LogLevel::Info, "UI initialized");
            HelloImGui::Run(runnerParams);
            return 0;
        }

        void ToolkitWindow() {
            int toolIdx = 0;
            for (auto& tool : v::appState.toolRegistry.tools) {
                if (ImGui::Selectable(tool->getLabel().c_str(), v::appState.toolRegistry.selectedIdx == toolIdx)) {
                    v::appState.toolRegistry.selectedIdx = toolIdx;
                }

                ++toolIdx;
            }
        }

        void ToolWindow() {
            v::appState.toolRegistry.tools[v::appState.toolRegistry.selectedIdx]->render();
        }
    };
}