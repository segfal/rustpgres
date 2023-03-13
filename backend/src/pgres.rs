use postgres::{Client, NoTls};


let mut client = Client::connect("host=localhost user=postgres", NoTls)?;


client.execute("CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
)", &[])?;


struct User {
    id: i32,
    name: String,
    email: String,
}


impl User {
    fn new(name: String, email: String) -> Self {
        Self {
            id: 0,
            name,
            email,
        }
    }

    fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_email(&self) -> &str {
        &self.email
    }


    fn insert(&mut self, client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
        client.execute("INSERT INTO users (name, email) VALUES ($1, $2)",
            &[&self.name, &self.email])?;

        let rows = client.query("SELECT id FROM users WHERE email = $1", &[&self.email])?;

        let id: i32 = rows[0].get(0);

        self.set_id(id);

        Ok(())
    }

    fn update(&mut self, client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
        client.execute("UPDATE users SET name = $1, email = $2 WHERE id = $3",
            &[&self.name, &self.email, &self.id])?;

        Ok(())
    }


    fn delete(&mut self, client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
        client.execute("DELETE FROM users WHERE id = $1", &[&self.id])?;

        Ok(())
    }


    fn get_all(client: &mut Client) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let mut users = Vec::new();

        let rows = client.query("SELECT id, name, email FROM users", &[])?;

        for row in rows {
            let user = User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
            };

            users.push(user);
        }

        Ok(users)
    }

    
}



