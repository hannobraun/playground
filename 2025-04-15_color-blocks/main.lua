local lick = require "lick"

local cell_size = 32;

function love.draw()
    for i = 1, 10, 1 do
        for j = 1, 16, 1 do
            love.graphics.rectangle(
                "fill",
                i * cell_size, j * cell_size,
                cell_size, cell_size
            )
        end
    end
end
