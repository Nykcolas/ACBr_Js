use std::path::PathBuf;
use libloading::{Library, Symbol};

#[link(name = "ACBrNFe64")]
extern "C" {
    fn NFE_Inicializar(caminhoDll: *const u8, caminhoConfig: *const u8) -> i32;
    fn NFE_Finalizar() -> i32;
    fn NFE_UltimoRetorno(chave: *const u8, protocolo: *const u8) -> i32;
    fn NFE_Nome(versao: *const u8, licenca: *const u8) -> i32;
    fn NFE_Versao(versao: *const u8, chave: *const u8) -> i32;
    fn NFE_ConfigLer(caminhoConfig: *const u8) -> i32;
    fn NFE_ConfigGravar(caminhoConfig: *const u8) -> i32;
    fn NFE_ImprimirEvento(evento: *const u8, path: *const u8) -> i32;
    fn NFE_ImprimirEventoPDF(evento: *const u8, path: *const u8) -> i32;
    fn NFE_SalvarEventoPDF(evento: *const u8, path: *const u8) -> i32;
    fn NFE_ImprimirInutilizacao(path: *const u8) -> i32;
    fn NFE_ImprimirInutilizacaoPDF(path: *const u8) -> i32;
    fn NFE_SalvarInutilizacaoPDF(path: *const u8) -> i32;
}

pub fn carregar_bibliotecas(caminho_dll: &str) -> Result<(), String> {
    let mut acbrnfe64_path = PathBuf::from(caminho_dll);
    acbrnfe64_path.push("ACBrNFe64.dll");
    let acbrnfe64_lib = match Library::new(acbrnfe64_path.to_str().unwrap()) {
        Ok(lib) => lib,
        Err(err) => return Err(format!("Erro ao carregar a biblioteca ACBrNFe64.dll: {}", err)),
    };

    // carregar outras bibliotecas aqui se necessário

    Ok(())
}