defmodule ServerControllerTest do
  use ExUnit.Case
  doctest ServerController

  test "greets the world" do
    assert ServerController.hello() == :world
  end
end
