local lick = require "lick"

local cell_size = 32;
local block_size = 30;

function love.draw()
    for i = 1, 10, 1 do
        for j = 1, 16, 1 do
            love.graphics.rectangle(
                "fill",
                i * cell_size, j * cell_size,
                block_size, block_size
            )
        end
    end
end
