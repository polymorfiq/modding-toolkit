defmodule ServerController.Controller do
  use GenServer

  def start_link({host, port}) do
    GenServer.start_link(__MODULE__, {host, port})
  end

  @impl true
  def init({host, port}) do
    {:ok, socket} = :gen_tcp.connect(host, port, packet: :line, active: false)

    {:ok, %{
      socket: socket
    }, {:continue, :initial_message}}
  end

  @impl true
  def handle_continue(:initial_message, state) do
    GenServer.cast(self(), :get_player_count)

    {:noreply, state}
  end

  @impl true
  def handle_cast(:get_player_count, %{socket: socket} = state) do
    :ok = :gen_tcp.send(socket, "get_players\n")
    {:ok, player_count} = :gen_tcp.recv(socket, 0)
    player_count = player_count
    |> List.to_string()
    |> String.trim()
    |> String.to_integer()

    player_count |> IO.inspect(label: "player_count")

    {:noreply, state}
  end
end
