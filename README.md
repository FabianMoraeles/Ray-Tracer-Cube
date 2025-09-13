# Mini Ray Tracer en Rust

Este proyecto es una implementación sencilla de un **ray tracer** en Rust.  
Renderiza una escena básica compuesta por:

- Un **cubo** (representado como un AABB, *Axis-Aligned Bounding Box*).  
- Un **plano** opcional que actúa como piso con un patrón de tablero de ajedrez.  
- Una **fuente de luz puntual** que genera iluminación difusa (modelo de Lambert).  
- Un **cielo degradado** (de blanco a azul).  

El resultado se guarda como una imagen PNG (`output.png`).

---

## 🚀 Ejecución

### Requisitos
- [Rust](https://www.rust-lang.org/) (versión estable o nightly).
- Dependencias de crate `image` para guardar la salida.

### Clonar y compilar
```bash
git clone <url-de-tu-repo>
cd <tu-repo>
cargo run --release
