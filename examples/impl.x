struct Player:
   name = str,
   age = 0
end

impl Player:
  func new(): Self
      Player(
        name = "",
        age = 0
      )
  end

  func add(&mut self, input_name: str, input_age: int)
    if name.len() < 30 then
       self.name += input_name
       self.age += input_age
    end
  end

  func return_data(&self): result[option[int], str]
     return Ok((self.age, self.name)
  end
end

func main()
  player = Player()
  player.new()
  player.add("player 1", 100)
  data = player.return_data()
end
