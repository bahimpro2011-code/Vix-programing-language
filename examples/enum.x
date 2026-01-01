enum Value:
  Int(int32),
  Float(float64),
  Text(str),
end

func main()
  mut array: array[Value] = array()


 array += Value.Int(10)
 array += Value.Float(3.5)
 array += Value.Text("Hello world")

 for value in array do
    match value:
      Value.Int(i) do print("Int:", i) end
      Value.Float(f) do print("Int:", i) end
      Value.Text(s) do print("Int:", i) end
    end
 end
end
