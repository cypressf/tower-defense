use piston_window::*;
const CAMERA_SPEED: f32 = 1.;

struct Game {
    // Stores the current state of the game, including the player's resources and the enemy units on the map
    state: GameState,
    // Stores the list of tower types that the player can build
    tower_types: Vec<TowerType>,
    // Stores the list of enemy types that will appear in the game
    enemy_types: Vec<EnemyType>,
}

impl Game {
    fn new() -> Self {
        // Initialize the game state and tower/enemy types
        let state = GameState::new();
        let tower_types = Vec::new();
        let enemy_types = Vec::new();

        Game {
            state,
            tower_types,
            enemy_types,
        }
    }

    fn update(&mut self) {
        // Update the game state, including spawning new enemies and advancing existing ones towards the player's base
        self.state.update(&self.enemy_types);

        // Check for collisions between towers and enemies and apply damage as necessary
        for tower in &self.state.towers {
            for enemy in &mut self.state.enemies {
                if tower.position.distance_to(&enemy.position) < tower.tower_type.range {
                    enemy.apply_damage(tower.tower_type.damage);
                }
            }
        }

        // Remove defeated enemies from the game
        let total_reward: i32 = self
            .state
            .enemies
            .iter()
            .filter(|enemy| !enemy.is_alive())
            .map(|enemy| enemy.enemy_type.reward)
            .sum();
        self.state.enemies.retain(|enemy| enemy.is_alive());
        self.state.resources += total_reward;

        // Check if the player has won or lost the game
        if self.state.enemies.is_empty() {
            println!("You win!");
        } else if self.state.lives <= 0 {
            println!("You lose!");
        }
    }
}

struct GameState {
    // Stores the player's current resources
    resources: i32,
    // Stores the player's current number of lives
    lives: i32,
    // Stores the list of towers that the player has placed
    towers: Vec<Tower>,
    // Stores the list of enemy units on the map
    enemies: Vec<Enemy>,
    camera_position: Point,
}

impl GameState {
    fn new() -> Self {
        // Initialize the game state with the player's starting resources and lives, and an empty list of towers and enemies
        let resources = 100;
        let lives = 10;
        let towers = Vec::new();
        let enemies = Vec::new();
        let camera_position = Point::new(0., 0.);

        GameState {
            resources,
            lives,
            towers,
            enemies,
            camera_position,
        }
    }

    fn update(&mut self, enemy_types: &Vec<EnemyType>) {
        // Spawn new enemies based on the current wave number
        let wave = self.enemies.len() / 10 + 1;
        for _ in 0..wave {
            self.enemies
                .push(Enemy::new(enemy_types[wave % enemy_types.len()].clone()));
        }

        // Advance all existing enemies towards the player's base
        for enemy in self.enemies.iter_mut() {
            enemy.advance();
        }
    }
}

#[derive(Clone)]
struct TowerType {
    // Stores the tower's name
    name: String,
    // Stores the tower's cost in resources
    cost: i32,
    // Stores the tower's damage per shot
    damage: i32,
    // Stores the tower's
    range: f32,
    // Stores the tower's rate of fire, in shots per second
    rate_of_fire: f32,
}

struct Tower {
    // Stores the tower's position on the map
    position: Point,
    // Stores the tower's type
    tower_type: TowerType,
}

impl Tower {
    fn new(position: Point, tower_type: TowerType) -> Self {
        Tower {
            position,
            tower_type,
        }
    }
}

#[derive(Clone)]
struct EnemyType {
    // Stores the enemy's name
    name: String,
    // Stores the enemy's maximum hit points
    max_hit_points: i32,
    // Stores the enemy's speed, in units per second
    speed: f32,
    // Stores the enemy's reward in resources upon defeat
    reward: i32,
}

struct Enemy {
    // Stores the enemy's position on the map
    position: Point,
    // Stores the enemy's current hit points
    hit_points: i32,
    // Stores the enemy's type
    enemy_type: EnemyType,
}

impl Enemy {
    fn new(enemy_type: EnemyType) -> Self {
        let position = Point::new(0.0, 0.0); // Place the enemy at the start of the map
        let hit_points = enemy_type.max_hit_points;
        Enemy {
            position,
            hit_points,
            enemy_type,
        }
    }

    fn advance(&mut self) {
        // Move the enemy towards the player's base
        self.position.x -= self.enemy_type.speed * time_since_last_frame();
    }

    fn apply_damage(&mut self, damage: i32) {
        self.hit_points -= damage;
    }

    fn is_alive(&self) -> bool {
        self.hit_points > 0
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    fn distance_to(&self, other: &Point) -> f32 {
        // Calculate the distance between two points using the Pythagorean theorem
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn time_since_last_frame() -> f32 {
    // Return the amount of time that has passed since the last frame
    // This would typically be implemented using a frame timer or delta time value
    0.01
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Tower Defense", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut glyphs = window
        .load_font("assets/fonts/Atkinson-Hyperlegible-Regular-102.otf")
        .unwrap();

    let enemy_type_1 = EnemyType {
        name: String::from("Goblin"),
        max_hit_points: 10,
        speed: 2.0,
        reward: 20,
    };

    let enemy_type_2 = EnemyType {
        name: String::from("Orc"),
        max_hit_points: 20,
        speed: 1.5,
        reward: 30,
    };

    let tower_type_1 = TowerType {
        name: String::from("Archer Tower"),
        cost: 50,
        damage: 5,
        range: 100.0,
        rate_of_fire: 1.0,
    };

    let tower_type_2 = TowerType {
        name: String::from("Mage Tower"),
        cost: 75,
        damage: 10,
        range: 200.0,
        rate_of_fire: 2.0,
    };

    let mut game = Game::new();
    game.enemy_types = vec![enemy_type_1, enemy_type_2];
    game.tower_types = vec![tower_type_1, tower_type_2];

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // Handle player input
            match key {
                Key::W => {
                    // Move the player's camera up
                    game.state.camera_position.y += CAMERA_SPEED;
                }
                Key::A => {
                    // Move the player's camera left
                    game.state.camera_position.x -= CAMERA_SPEED;
                }
                Key::S => {
                    // Move the player's camera down
                    game.state.camera_position.y -= CAMERA_SPEED;
                }
                Key::D => {
                    // Move the player's camera right
                    game.state.camera_position.x += CAMERA_SPEED;
                }
                Key::Space => {
                    // Place a tower at the player's current position
                    let tower_type = &game.tower_types[0]; // For simplicity, use the first tower type in the list
                    if game.state.resources >= tower_type.cost {
                        game.state
                            .towers
                            .push(Tower::new(game.state.camera_position, tower_type.clone()));
                        game.state.resources -= tower_type.cost;
                    }
                }
                _ => {}
            }
        }
        window.draw_2d(&event, |c, g, _device| {
            clear([1.0; 4], g);

            // Draw the player's base
            rectangle([0.0, 0.5, 0.0, 1.0], [0.0, 0.0, 50.0, 50.0], c.transform, g);

            // Draw the player's resources
            text(
                [0.0, 0.0, 0.0, 1.0],
                20,
                &format!("Resources: {}", game.state.resources),
                &mut glyphs,
                c.transform.trans(0.0, 50.0),
                g,
            )
            .unwrap();

            // Draw the player's lives
            text(
                [0.0, 0.0, 0.0, 1.0],
                20,
                &format!("Lives: {}", game.state.lives),
                &mut glyphs,
                c.transform.trans(0.0, 70.0),
                g,
            )
            .unwrap();

            // Draw the player's towers
            for tower in game.state.towers.iter() {
                let transform = c
                    .transform
                    .trans(tower.position.x.into(), tower.position.y.into());
                ellipse([0.5, 0.5, 0.5, 1.0], [0.0, 0.0, 25.0, 25.0], transform, g);
            }

            // Draw the enemy units
            for enemy in game.state.enemies.iter() {
                let transform = c
                    .transform
                    .trans(enemy.position.x.into(), enemy.position.y.into());
                rectangle([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 25.0, 25.0], transform, g);
            }
        });

        event.update(|_| {
            // Update the game state
            game.update();
        });
    }
}
