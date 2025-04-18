local field_width = 10
local field_height = 16

local block_size = 32
local block_offset = 1
local block_size_inner = block_size - 2 * block_offset

local drop_tick_s = 0.25

local field = {}
local dt_acc = 0

local move_left = false
local move_right = false
local drop_now = false

function love.load()
    field = empty_field()
    spawn_block()
end

function love.keypressed(key)
    if key == "escape" then
        love.event.quit()
    end

    move_left = key == "left" or key == "a"
    move_right = key == "right" or key == "d"

    drop_now = key == "space" or key == "down"
end

function love.update(dt)
    dt_acc = dt_acc + dt

    local input_ready = false
    local drop_ready = false

    if dt_acc >= 0.1 then
        input_ready = true
    end
    if dt_acc >= drop_tick_s then
        drop_ready = true
        dt_acc = dt_acc - drop_tick_s
    end

    local field_next = empty_field()

    for x = 1, field_width, 1 do
        for y = 1, field_height, 1 do
            if field[x][y] then
                local x_offset = 0
                local y_offset = 0

                if input_ready and field[x][y].control then
                    if move_left and x > 1 then
                        x_offset = -1
                        move_left = false
                    end
                    if move_right and x < field_width then
                        x_offset = 1
                        move_right = false
                    end
                end
                if drop_ready then
                    if drop_now then
                        y_offset = distance_to_floor(x, y)
                        field[x][y].control = false
                        drop_now = false
                    elseif y < field_height and distance_to_floor(x, y) > 0 then
                        y_offset = 1
                    else
                        field[x][y].control = false
                    end
                end

                field_next[x + x_offset][y + y_offset] = field[x][y]
            end
        end
    end

    field = field_next

    local controlled_blocks_exist = false
    for x = 1, field_width, 1 do
        for y = 1, field_height, 1 do
            local block = field[x][y]

            if block then
                controlled_blocks_exist =
                    controlled_blocks_exist or block.control
            end
        end
    end

    if not controlled_blocks_exist then
        spawn_block()
    end
end

function love.draw()
    for i = 1, field_width, 1 do
        for j = 1, field_height, 1 do
            local x = (i - 1) * block_size
            local y = (j - 1) * block_size
            
            if field[i][j] then
                love.graphics.rectangle(
                    "fill",
                    x, y,
                    block_size_inner, block_size_inner
                )
            end
        end
    end
end

function empty_field()
    local empty_field = {}

    for i = 1, field_width, 1 do
        empty_field[i] = {}

        for j = 1, field_height, 1 do
            empty_field[i][j] = nil
        end
    end

    return empty_field
end

function spawn_block()
    local x = 5
    local y = 1

    if field[x][y] then
        love.event.quit()
    end

    field[x][y] = { control = true }
end

function distance_to_floor(x, y)
    local distance = 0

    for i = y + 1, field_height, 1 do
        if field[x][i] then
            break
        else
            distance = distance + 1
        end
    end

    return distance
end
