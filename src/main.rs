

use gamelads::gamelad::Gamelad;

// https://www.youtube.com/watch?v=HyzD8pNlpwI
// https://gbdev.io/gb-opcodes//optables/
// https://github.com/gbdev/awesome-gbdev
// https://eldred.fr/gb-asm-tutorial/data_manip.html#ld
// https://gbdev.io/pandocs/


fn main() -> Result<(), String> {

    let filename = "./roms/01-special.gb";//"./asm/bin/load_reg";
    
    let mut gamelad = Gamelad::new(filename);

    gamelad.run();

    Ok(())
}