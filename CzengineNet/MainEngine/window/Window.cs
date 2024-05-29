using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MainEngine.window {
    public abstract class Window {
        protected bool sustain = false;
        protected WindowProperties properties;
        public abstract void RunLoop();
        public abstract void Configure(WindowProperties windowProperties);
    }
}
