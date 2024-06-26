#include "window.hpp"
#include <algorithm>

namespace CzaraEngine {
    Window::Window() :
        children(), width(800), height(640), name("Czengine++"), x_window_offset(0), y_window_offset(0) {}
    Window::Window(const WindowProperties &properties) : 
        children(), width(properties.width), height(properties.height), name(properties.name),
        x_window_offset(properties.x_window_offset), y_window_offset(properties.y_window_offset) {}
    Window::Window(const Window& window) :
        children(window.children), width(window.width), height(window.height), name(window.name),
        x_window_offset(window.x_window_offset), y_window_offset(window.y_window_offset) {
        }

    Window::~Window() {
        clearChildrenVector();
    }

    Window& Window::operator=(const Window& window) {
        clearChildrenVector();
        this->children = window.children;
        this->name = window.name;
        this->width = window.width;
        this->height = window.height;
        this->x_window_offset = window.x_window_offset;
        this->y_window_offset = window.y_window_offset;
        return *this;
    }

    void Window::clearChildrenVector() {
        // if (!children.empty()) {
        //     // std::for_each(children.cbegin(), children.cend(), [](std::unique_ptr<Window> child) {
        //     //     child.reset();
        //     // });
        //     children.clear();
        // }
    }
}
