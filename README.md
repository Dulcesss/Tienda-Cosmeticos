El proyecto consiste en el desarrollo de un programa en Solana utilizando el framework Anchor, cuyo objetivo es simular el funcionamiento básico de una tienda de cosméticos de belleza dentro de la blockchain. Este programa permite administrar productos cosméticos mediante diferentes funciones que permiten crear una tienda, agregar productos, eliminarlos, visualizar los existentes, modificar su disponibilidad y consultar el total de productos registrados.

El primer paso en el desarrollo fue la creación del programa en Anchor, donde se definió el identificador del programa mediante declare_id. Posteriormente se creó el módulo principal del programa donde se implementaron las funciones que permiten interactuar con la tienda.

Después se definió la estructura principal llamada Tienda, la cual almacena la información básica del negocio. Esta estructura guarda la clave pública del propietario (owner), el nombre de la tienda y una lista de productos cosméticos almacenados en un vector. Cada producto se representa mediante la estructura Cosmetico, que contiene el nombre del producto, su precio y un indicador booleano que señala si el producto se encuentra disponible o no.

Posteriormente se implementaron las funciones principales del programa:

crear_tienda: Permite inicializar una nueva tienda asociada al propietario de la cuenta. Aquí se crea la estructura de la tienda y se inicializa el vector de productos.

agregar_producto: Permite añadir un nuevo cosmético a la lista de productos de la tienda.

eliminar_producto: Busca un producto por su nombre y lo elimina del vector si existe.

ver_productos: Muestra en consola la lista de productos registrados en la tienda.

alternar_disponibilidad: Cambia el estado de disponibilidad de un producto, permitiendo activarlo o desactivarlo.

total_productos: Muestra la cantidad total de productos almacenados en la tienda.

Para garantizar la seguridad del sistema, se implementó una validación del propietario, de modo que solo el dueño de la tienda pueda modificar la información. En caso contrario, el programa devuelve un error definido en el enum Errores.

Además, se definieron las cuentas necesarias mediante las estructuras NuevaTienda y NuevoProducto, las cuales especifican los permisos y requisitos que deben cumplir las cuentas que interactúan con el programa. También se utilizaron Program Derived Addresses (PDA) para crear de manera segura la cuenta de la tienda dentro de la blockchain.

Finalmente, se realizó la interacción con el programa mediante JavaScript en Solana Playground, donde se ejecutaron diferentes instrucciones para mostrar la dirección de la billetera, consultar el balance en SOL, crear la tienda y agregar productos cosméticos.

Gracias a este proceso se logró construir un sistema sencillo de gestión de productos dentro de la blockchain, aplicando conceptos importantes como cuentas en Solana, estructuras de datos, validación de usuarios, y manipulación de vectores en Rust.
