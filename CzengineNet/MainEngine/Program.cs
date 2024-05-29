// See https://aka.ms/new-console-template for more information
using MainEngine.window;

SdlWindow window = new(new WindowProperties(){
    name = "Czengine.NET",
    width = 1080,
    height = 720,
    xOffset = 50,
    yOffset = 50,
});
window.RunLoop();
