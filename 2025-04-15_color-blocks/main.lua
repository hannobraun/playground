local lick = require "lick"

function love.draw()
    for i = 1, 10, 1 do
        for j = 1, 16, 1 do
            love.graphics.rectangle(
                "fill",
                i * 32, j * 32,
                32, 32
            )
        end
    end
end
