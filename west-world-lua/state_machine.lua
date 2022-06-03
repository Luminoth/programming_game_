-------------------------------------------------------------------------------
-- GoHome state
-------------------------------------------------------------------------------
State_GoHome = {}

State_GoHome["Enter"] = function(miner)
    print("[Lua]: Walkin home in the hot n' thusty heat of the desert")
end


State_GoHome["Execute"] = function(miner, state_machine)
    print("[Lua]: Back at the shack. yer siree!")

    if miner:is_fatigued() then
        state_machine:change_state(miner, "State_Sleep")
    else
        state_machine:change_state(miner, "State_GoToMine")
    end
end

State_GoHome["Exit"] = function(miner)
    print("[Lua]: Puttin' mah boots on n' gettin' ready for a day at the mine")
end

-------------------------------------------------------------------------------
-- Sleep state
-------------------------------------------------------------------------------
State_Sleep = {}

State_Sleep["Enter"] = function(miner)
    print("[Lua]: Miner " .. miner:name() .. " is dozin off")
end

State_Sleep["Execute"] = function(miner, state_machine)
    if miner:is_fatigued() then
        print("[Lua]: ZZZZZZ... ")
        miner:rest()
    else
        state_machine:change_state(miner, "State_GoToMine")
    end
end

State_Sleep["Exit"] = function(miner)
    print("[Lua]: Miner " .. miner:name() .. " is feelin' mighty refreshed!")
end

-------------------------------------------------------------------------------
-- GoToMine state
-------------------------------------------------------------------------------
State_GoToMine = {}

State_GoToMine["Enter"] = function(miner)
    print("[Lua]: Miner " .. miner:name() .. " enters goldmine")
end

State_GoToMine["Execute"] = function(miner, state_machine)
    miner:mine_gold(2)

    print("[Lua]: Miner " .. miner:name() .. " has got " .. miner:GoldCarried() .. " nuggets")

    if miner:gold_carried() > 4 then
        print("[Lua]: Miner " .. miner:name() .. " decides to go home, with his pockets full of nuggets")
        state_machine:change_state(miner, "State_GoHome")
    end
end

State_GoToMine["Exit"] = function(miner)
    print("[Lua]: Miner " .. miner:name() .. " exits goldmine")
end
