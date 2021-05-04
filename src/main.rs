use rand::Rng;
use text_io::read;

// Dice structure
struct Dice
{
    amount: i32,
    sides: i32,
}

impl Dice
{
    fn display_range(&self) // Displays the dice roll range
    {
        println!("{} to {}", self.amount, self.max_roll());
    }

    fn max_roll(&self) -> i32 // Gets the max roll possible
    {
        self.amount * self.sides
    }

    fn roll(&self) -> i32 // Rolls the dice, returns the roll
    {
        let mut rng = rand::thread_rng();
        rng.gen_range(self.amount..self.max_roll())
    }

    fn create_dice() -> Dice // Creates the dice with user input
    {
        println!("Enter amount of die: ");
        let init_amount: i32 = read!();
        println!("Enter amount of sides per die: ");
        let init_sides: i32 = read!();

        Dice
        {
            amount: init_amount,
            sides: init_sides,
        }
    }
}

fn main()
{
    println!("Welcome to Frozen Marina's Rust Dice Roll!");

    let menu_options = ["Create Die", "Quit"];
    let mut x: usize = 0; // Counter to properly print out menu options

    loop
    {
        x = 0; // Reset counter upon loop so that it doesn't go past
               // the length of the array of options.

        for _element in menu_options.iter() // Print out the Main Menu
        {
            x += 1;
            println!("[{}] - {}", x, menu_options[x - 1]);
        }

        let choice: i16 = read!();

        if choice == 1
        {
            let damage_dice = Dice::create_dice(); // Create the Dice

            let dice_options = ["Check Range", "Roll Die", "Return"];

            x = 0;

            loop
            {
                x = 0;

                for _element in dice_options.iter() // Prints out dice utility menu.
                {
                    x += 1;
                    println!("[{}] - {}", x, dice_options[x - 1]);
                }

                let dice_choice: i16 = read!();

                if dice_choice == 1
                {
                    damage_dice.display_range();
                } else if dice_choice == 2
                {
                    println!("You roll the dice...{}", damage_dice.roll());
                } else if dice_choice == 3 // If user chooses to return, break current loop.
                {
                    break;
                } else
                {
                    println!("That is not a valid option!\n");
                }
            }

        } else if choice == 2 // If user chooses to quit, break loop.
        {
            break;
        } else
        {
            println!("That is not a valid option!\n");
        }
    }
}
