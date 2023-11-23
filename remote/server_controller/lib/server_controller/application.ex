defmodule ServerController.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      # Starts a worker by calling: ServerController.Worker.start_link(arg)
      {ServerController.Controller, {{192, 168, 86, 111}, 4951}}
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: ServerController.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
