
using MainEngine.window;
using SDL2;

namespace MainEngine.window {
    public class SdlWindow : Window {
        private nint window;
        private nint renderer;
        public SdlWindow(WindowProperties properties) {
            this.properties = properties;
            if (SDL.SDL_Init(SDL.SDL_INIT_VIDEO) < 0) {
                Console.Error.WriteLine($"There was an issue initializing SDL. {SDL.SDL_GetError()}");
            }
            window = SDL.SDL_CreateWindow(
                properties.name,
                properties.xOffset,
                properties.yOffset,
                properties.width,
                properties.height,
                SDL.SDL_WindowFlags.SDL_WINDOW_SHOWN
            );
            if (window == IntPtr.Zero) {
                Console.Error.WriteLine($"There was an issue creating the window. {SDL.SDL_GetError()}");
            }
            renderer = SDL.SDL_CreateRenderer(
                window, 
                -1, 
                SDL.SDL_RendererFlags.SDL_RENDERER_ACCELERATED 
                | SDL.SDL_RendererFlags.SDL_RENDERER_PRESENTVSYNC
            );
            if (renderer == IntPtr.Zero) {
                Console.Error.WriteLine($"There was an issue creating the renderer. {SDL.SDL_GetError()}");
            }
        }

        ~SdlWindow() {
            SDL.SDL_DestroyRenderer(renderer);
            SDL.SDL_DestroyWindow(window);
            SDL.SDL_Quit();
        }

        public override void Configure(WindowProperties windowProperties) {
            SDL.SDL_SetWindowSize(window, windowProperties.width, windowProperties.height);
            SDL.SDL_SetWindowTitle(window, windowProperties.name);
            this.properties = windowProperties;
        }

        public override void RunLoop() {
            sustain = true;
            while ( sustain ) {
                while(SDL.SDL_PollEvent(out SDL.SDL_Event e) == 1) {
                    switch (e.type) {
                        case SDL.SDL_EventType.SDL_QUIT:
                            sustain = false;
                            break;
                    }
                }

                // Sets the color that the screen will be cleared with.
                if (SDL.SDL_SetRenderDrawColor(renderer, 135, 206, 235, 255) < 0) {
                    Console.Error.WriteLine($"There was an issue with setting the render draw color. {SDL.SDL_GetError()}");
                }

                // Clears the current render surface.
                if (SDL.SDL_RenderClear(renderer) < 0) {
                    Console.Error.WriteLine($"There was an issue with clearing the render surface. {SDL.SDL_GetError()}");
                }

                // Switches out the currently presented render surface with the one we just did work on.
                SDL.SDL_RenderPresent(renderer);
            }
        }
    }
}
