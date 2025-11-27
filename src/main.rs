use std::{
    env::args,
    error::Error,
    fs::{File, create_dir_all},
    io::BufReader,
    path::Path,
};

use image::{AnimationDecoder, codecs::gif::GifDecoder};

fn main() -> Result<(), Box<dyn Error>> {
    //El primer argumento es el .gif
    let argumento = args().nth(1).expect("Se esperaba al menos un argumento");
    //El segundo argumento es el path al que enviar las cositas
    let salida = args().nth(2).expect("Se esperaba un ruta de salida");
    let ruta_base = Path::new(&salida);
    println!("Abriendo directorio {}...", salida);
    create_dir_all(ruta_base)?;
    println!("Abriendo archivo {}... ", argumento);
    let archivo = File::open(argumento)?;
    let lector = BufReader::new(archivo);
    let decodificador = GifDecoder::new(lector)?;
    println!("Decodificando el gif...");
    let frames = decodificador.into_frames();
    for (k, frame_result) in frames.enumerate() {
        println!("Obteniendo el frame {}...", k);
        let result = frame_result?;
        let imagen = result.buffer();
        imagen.save(ruta_base.join(format!("{:0>3}.png", k)))?;
    }
    Ok(())
}
