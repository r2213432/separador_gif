# 🦀 GIF Frame Extractor

Una herramienta de línea de comandos (CLI) escrita en **Rust** para descomponer archivos `.gif` animados en imágenes `.png` individuales.

Este proyecto fue desarrollado como un ejercicio práctico para entender el manejo de archivos (I/O), el uso de iteradores y el ecosistema de procesamiento de imágenes en Rust.

## 🚀 Características

* **Decodificación eficiente:** Utiliza el crate `image` para leer el flujo de datos frame a frame.
* **Gestión automática de directorios:** Crea la carpeta de salida si no existe.
* **Ordenamiento correcto:** Guarda los archivos con numeración formateada (`000.png`, `001.png`...) para asegurar que el sistema operativo los ordene cronológicamente.
* **Feedback en consola:** Informa al usuario sobre el estado del proceso en tiempo real.

## 🛠️ Instalación y Compilación

Proximamente => apartado de releases

📋 Para compilar desde el proyecto fuente

Necesitas tener instalado **Rust** y **Cargo** en tu sistema.

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```

Clonas el proyecto desde github y entras dentro

```bash
git clone https://github.com/r2213432/separador_gif.git
cd separador_gif
```

Ahora tienes 2 opciones:

* Compilarlo y dejarlo dentro de la path de .cargo:

```bash
cargo install --path .
```

* Solo compilarlo y añadir el ejecutable a la path a tu modo

```bash
cargo build --release --locked
```

Este ultimo comando creara la carpeta **target/**, dentro de esta deberia de haberse guardado el ejecutable en la carpeta **release/** con nombre "**separador_gif**"

```bash
cd target/release/
#Lanzar el ejecutable para probarlo
./separador_gif <archivo.gif> <carpeta_salida>
```
