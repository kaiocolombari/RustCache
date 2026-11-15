use crate::cache::SharedCache;

pub fn executar_tarefa(id: usize, cache: SharedCache<String, i32>) {

    {
        let leitor = cache.read().unwrap();
        if let Some(valor) = leitor.get(&"inicial".to_string()) {
            println!("Thread {id} leu: {valor}");
        }
    } 

    if id % 2 == 0 {
        let mut escritor = cache.write().unwrap();
        escritor.insert(format!("chave_{id}"), (id * 10) as i32);
        println!("Thread {id} gravou no cache!");
    }
    
}
