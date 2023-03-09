#include "ui.hpp"

#include "tool-template.hpp"
#include "dht-viewer.hpp"

#include <iostream>

int main(int , char *[]) {
    v::appState.toolRegistry.tools.push_back(std::make_shared<v::ToolTemplate>());
    v::appState.toolRegistry.tools.push_back(std::make_shared<v::DHTViewer>());

	v::UI ui;
    return ui.Run();
}