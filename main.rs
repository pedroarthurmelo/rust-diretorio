//////////////////////////////////// PERMISSAO /////////////////////////////////////////////
struct Permissao {
    leitura: bool,    // Permissão de leitura
    escrita: bool,    // Permissão de escrita
    execucao: bool,   // Permissão de execução
}

impl Permissao {
    // implementação para inicializar uma nova permissão com os valores de leitura, escrita e execução
    fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao {
        Permissao {
            leitura,
            escrita,
            execucao,
        }
    }

    // Método para converter permissões em um valor octal
    fn octal(&self) -> u8 {
        let a: u8 = if self.leitura {1} else {0}; // Define o bit de leitura
        let b: u8 = if self.escrita {1} else {0}; // Define o bit de escrita
        let c: u8 = if self.execucao {1} else {0}; // Define o bit de execução

        // Combina os bits em um valor octal
        match (a, b, c) {
            (0, 0, 0) => 0,
            (0, 0, 1) => 1,
            (0, 1, 0) => 2,
            (0, 1, 1) => 3,
            (1, 0, 0) => 4,
            (1, 0, 1) => 5,
            (1, 1, 0) => 6,
            (1, 1, 1) => 7,
            _ => 20, // apenas debug
        }
    }

    // Retorna uma string representando a permissão no formato "rwx"
    fn rwx(&self) -> String {
        let r = if self.leitura {'r'} else {'-'};
        let w = if self.escrita {'w'} else {'-'};
        let x = if self.execucao {'x'} else {'-'};
        format!("{r}{w}{x}")
    }

    // Combina as permissões de dono, grupo e outros em strings octal e rwx completas
    fn octal_e_rwx_total(dono: &Permissao, grupo: &Permissao, outros: &Permissao) -> (String, String) {
        let octal = format!("{}{}{}", dono.octal(), grupo.octal(), outros.octal());
        let rwx = format!("{}|{}|{}", dono.rwx(), grupo.rwx(), outros.rwx());
        (octal, rwx)
    }
}

//////////////////////////////////// USUARIO /////////////////////////////////////////////
struct Usuario {
    nome: String,
    uid: u16,
    grupos: Vec<Grupo>, // vetor para armazenar os grupos do usuário
}

impl Usuario {
    // implementação para criar um novo usuário com nome e uid
    fn new(nome: String, uid: u16) -> Usuario {
        Usuario {
            nome,
            uid,
            grupos: Vec::new(),
        }
    }

    // Adiciona um grupo ao vetor de grupos do usuário
    fn adiciona_grupo(&mut self, grupo: Grupo) {
        self.grupos.push(grupo); //push adiciona no final do vetor o usuario novo
    }

    // Remove um grupo do vetor de grupos do usuário, filtrando pelo nome do grupo
    fn remove_grupo(&mut self, nome_grupo: &str) {
        self.grupos.retain(|g| g.nome != nome_grupo);  //retain filtra os 
    }

    // Lista todos os grupos aos quais o usuário pertence
    fn listar_grupos(&self) {
        for grupo in &self.grupos {
            println!("Grupo: {} (gid: {})", grupo.nome, grupo.gid);
        }
    }
}

//////////////////////////////////// GRUPO /////////////////////////////////////////////
struct Grupo {
    nome: String,
    gid: u16,
    membros: Vec<Usuario>,
}

impl Grupo {
    // Construtor para criar um novo grupo com nome e gid
    fn new(nome: String, gid: u16) -> Grupo {
        Grupo {
            nome,
            gid,
            membros: Vec::new(),
        }
    }

    // Adiciona um usuário ao grupo
    fn adiciona_membro(&mut self, usuario: Usuario) {
        self.membros.push(usuario);
    }

    // Remove um membro do grupo pelo nome do usuário
    fn remover_membro(&mut self, nome_usuario: &str) {
        self.membros.retain(|u| u.nome != nome_usuario);
    }

    // Lista todos os membros do grupo
    fn listar_grupos(&self) {
        for membro in &self.membros {
            println!("Membro: {} (uid: {})", membro.nome, membro.uid);
        }
    }
}

//////////////////////////////////// ARQUIVO /////////////////////////////////////////////
struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: (Permissao, Permissao, Permissao), // Permissões para dono, grupo, e outros
    usuario: Usuario, // Dono do arquivo
    grupo: Grupo, // Grupo associado ao arquivo
}

impl Arquivo {
    // Método para criar um novo arquivo com permissões padrão
    fn new(nome: String, tamanho: u64, usuario: Usuario, grupo: Grupo) -> Arquivo {
        // Permissões padrão: leitura = false, escrita = true, execução = false
        let permissoes = (
            Permissao::new(false, true, false), // Permissão para dono
            Permissao::new(false, true, false), // Permissão para grupo
            Permissao::new(false, true, false), // Permissão para outros
        );

        Arquivo {
            nome,
            tamanho,
            permissoes,
            usuario,
            grupo,
        }
    }

    // Método para alterar a permissão do arquivo
    fn alterar_permissao(&mut self, nova_permissao: (Permissao, Permissao, Permissao)) {
        self.permissoes = nova_permissao;
    }

    // Método para exibir informações do arquivo
    fn stat(&self) {
        let (octal, rwx) = Permissao::octal_e_rwx_total(&self.permissoes.0, &self.permissoes.1, &self.permissoes.2);
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {}", self.tamanho);
        println!("Permissões: ({}/{})", octal, rwx);
        println!("Uid: {}", self.usuario.uid);
        println!("Gid: {}", self.grupo.gid);
    }
}

//////////////////////////////////// DIRETORIO /////////////////////////////////////////////
struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>, // Vetor de arquivos no diretório
    permissoes: (Permissao, Permissao, Permissao), // Permissões do diretório
    dono: Usuario, // Dono do diretório
}

impl Diretorio {
    // Método para criar um novo diretório
    fn new(nome: String, permissoes: (Permissao, Permissao, Permissao), dono: Usuario) -> Diretorio {
        Diretorio {
            nome,
            arquivos: Vec::new(),
            permissoes,
            dono,
        }
    }

    // Adiciona um arquivo ao diretório
    fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }

    // Remove um arquivo do diretório pelo nome
    fn remove_arquivo(&mut self, nome_arquivo: &str) {
        self.arquivos.retain(|a| a.nome != nome_arquivo);
    }

    // Lista o conteúdo do diretório
    fn listar_conteudo(&self) {
        if self.arquivos.is_empty() {
            println!("O diretório {} está vazio", self.nome);
        } else {
            println!("Conteúdo do diretório {}:", self.nome);
            for arquivo in &self.arquivos {
                println!("Arquivo: {} (tamanho: {} bytes)", arquivo.nome, arquivo.tamanho);
            }
        }
    }
}

fn main() {
    let dono = Permissao::new(true, true, true); // Permissões do dono: rwx
    let grupo = Permissao::new(true, true, false); // Permissões do grupo: rw-
    let outros = Permissao::new(false, false, true); // Permissões dos outros: --x

    // Mostra a combinação octal e rwx das permissões do arquivo
    let (octal, rwx) = Permissao::octal_e_rwx_total(&dono, &grupo, &outros);
    println!("Permissões: ({}/{})", octal, rwx);

    let mut grupo1 = Grupo::new(String::from("Admin"), 1);
    let usuario1 = Usuario::new(String::from("João"), 1001);
    grupo1.adiciona_membro(usuario1);

    println!("Membros do grupo {}:", grupo1.nome);
    grupo1.listar_grupos();

    grupo1.remover_membro("João");
    println!("Membros do grupo {} após remoção:", grupo1.nome);
    grupo1.listar_grupos();
}
