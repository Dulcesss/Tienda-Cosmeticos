// Importa la librería principal de Anchor que incluye macros y utilidades necesarias
use anchor_lang::prelude::*;

// Identificador único del programa en la blockchain de Solana
declare_id!("9Ht3Fp8YwQd6Lm2Jv4Nx7Ra1TzKp5CsG8BuD3Xe1Vt9");

// Macro que define el programa de Anchor
#[program]
pub mod tienda_cosmeticos {
    use super::*;

    // Función que crea una nueva tienda de cosméticos
    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {

        // Obtiene la clave pública del propietario
        let owner_id = context.accounts.owner.key();

        // Imprime en los logs de Solana el ID del propietario
        msg!("Owner id: {}", owner_id);

        // Crea un vector vacío donde se almacenarán los cosméticos
        let productos: Vec<Cosmetico> = Vec::new();

        // Inicializa la cuenta tienda con sus datos
        context.accounts.tienda.set_inner(Tienda {
            owner: owner_id,
            nombre,
            productos,
        });

        // Indica que la operación fue exitosa
        Ok(())
    }

    // Función para agregar un cosmético a la tienda
    pub fn agregar_producto(context: Context<NuevoProducto>, nombre: String, precio: u16) -> Result<()> {

        // Verifica que quien ejecuta la función sea el dueño de la tienda
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Crea un nuevo cosmético
        let cosmetico = Cosmetico {
            nombre,
            precio,
            disponible: true,
        };

        // Agrega el cosmético al vector de productos
        context.accounts.tienda.productos.push(cosmetico);

        Ok(())
    }

    // Función para eliminar un producto
    pub fn eliminar_producto(context: Context<NuevoProducto>, nombre: String) -> Result<()> {

        // Verifica que quien ejecuta la función sea el dueño
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Obtiene los productos de la tienda
        let productos = &mut context.accounts.tienda.productos;

        // Recorre todos los productos
        for i in 0..productos.len() {

            // Si encuentra el producto con el nombre indicado
            if productos[i].nombre == nombre {

                // Elimina el producto del vector
                productos.remove(i);

                // Muestra mensaje en los logs
                msg!("Producto {} eliminado!", nombre);

                return Ok(());
            }
        }

        // Error si el producto no existe
        Err(Errores::ProductoNoExiste.into())
    }

    // Función para mostrar todos los productos
    pub fn ver_productos(context: Context<NuevoProducto>) -> Result<()> {

        // Verifica que quien ejecuta la función sea el dueño
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Muestra la lista de cosméticos en los logs
        msg!("Lista de cosméticos: {:#?}", context.accounts.tienda.productos);

        Ok(())
    }

    // Función para cambiar la disponibilidad de un producto
    pub fn alternar_disponibilidad(context: Context<NuevoProducto>, nombre: String) -> Result<()> {

        // Verifica que quien ejecuta la función sea el dueño
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Obtiene los productos de la tienda
        let productos = &mut context.accounts.tienda.productos;

        // Recorre los productos
        for i in 0..productos.len() {

            let estado = productos[i].disponible;

            // Si encuentra el producto
            if productos[i].nombre == nombre {

                // Cambia el estado de disponibilidad
                let nuevo_estado = !estado;
                productos[i].disponible = nuevo_estado;

                // Muestra el nuevo estado
                msg!("El cosmético {} ahora tiene disponibilidad: {}", nombre, nuevo_estado);

                return Ok(());
            }
        }

        // Error si el producto no existe
        Err(Errores::ProductoNoExiste.into())
    }

    // Función para mostrar el total de productos
    pub fn total_productos(context: Context<NuevoProducto>) -> Result<()> {

        // Cuenta los productos en el vector
        let total = context.accounts.tienda.productos.len();

        // Muestra el total en los logs
        msg!("La tienda tiene {} cosméticos registrados", total);

        Ok(())
    }
}

// Enum que define los posibles errores
#[error_code]
pub enum Errores {

    // Error cuando alguien que no es el dueño intenta modificar la tienda
    #[msg("Error, no eres el propietario de la tienda")]
    NoEresElOwner,

    // Error cuando el producto no existe
    #[msg("El producto no existe")]
    ProductoNoExiste,
}

// Estructura que representa la tienda
#[account]
#[derive(InitSpace)]
pub struct Tienda {

    // Clave pública del propietario
    owner: Pubkey,

    // Nombre de la tienda
    #[max_len(60)]
    nombre: String,

    // Lista de cosméticos
    #[max_len(10)]
    productos: Vec<Cosmetico>,
}

// Estructura que representa cada cosmético
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Cosmetico {

    // Nombre del cosmético
    #[max_len(60)]
    nombre: String,

    // Precio del cosmético
    precio: u16,

    // Indica si está disponible
    disponible: bool,
}

// Contexto necesario para crear una tienda
#[derive(Accounts)]
pub struct NuevaTienda<'info> {

    // Propietario de la tienda
    #[account(mut)]
    pub owner: Signer<'info>,

    // Cuenta donde se almacenará la tienda
    #[account(
        init,
        payer = owner,
        space = Tienda::INIT_SPACE + 8,
        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    // Programa del sistema de Solana
    pub system_program: Program<'info, System>,
}

// Contexto para manejar productos
#[derive(Accounts)]
pub struct NuevoProducto<'info> {

    // Propietario que ejecuta la acción
    pub owner: Signer<'info>,

    // Cuenta de la tienda que se modificará
    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}
