INSTRUCCIONES PARA EJECUTAR EL PROYECTO (comandos marcados con ‼️)

📥 Paso 1: Instalar Requisitos

    Node.js (para el frontend):

        Descargar instalador desde: https://nodejs.org/es/download/

        Hacer doble clic en el archivo descargado y seguir los pasos.

    Rust (para el backend):

        Descargar desde: https://www.rust-lang.org/es/tools/install

        Ejecutar en terminal (Windows: CMD o PowerShell):
        bash

            ‼️ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

        Presionar Enter para instalar con opciones predeterminadas.

🖥️ Paso 2: Ejecutar el Proyecto

    Backend (Terminal 1):

        Abrir PowerShell o CMD como administrador.

        Navegar a la carpeta del backend:
        bash

            ‼️ cd ruta\al\proyecto\backend

        Iniciar el servidor:
        bash

            ‼️ cargo run --release

        Esperar a que aparezca: Servidor iniciado en http://localhost:8080

    Frontend (Terminal 2):

        Abrir otra ventana de PowerShell o CMD.

        Navegar a la carpeta del frontend:
        bash

            ‼️ cd ruta\al\proyecto\frontend

        Instalar dependencias (solo primera vez):
        bash

            ‼️ npm install

        Iniciar la interfaz:
        bash

            ‼️ npm start

        Se abrirá automáticamente en: http://localhost:3000

⚠️ Si hay errores:

    Verificar que ambos servidores estén en ejecución.

    Cerrar y volver a abrir las terminales si es necesario.

    Asegurarse de que no haya otros programas usando los puertos 3000 o 8080.
