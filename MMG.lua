local info = {
    techmodname = "",
    modname = "",
    shortdescription = "",
    repository = "",
    contentdb = "",
    license_code = "",
    license_media = "",
    depends = "",
    optional_depends = "",
    type = "",
    game = "",
    game_link = "",
    download = "",
    screenshots = {}
}

local answer

local function cprint(...)
    io.write(...)
end

local function getinput(type)
    local temp
    
    if type == "line" then
        cprint("\n\n> ")
        temp = io.read("*line")
    elseif type == "lines" then
        cprint("\n\nMultiline input-> ")

        local lines = ""
        temp = io.read("*line")

        while temp ~= "" do
            lines = lines .. temp .. "\n"
            temp = io.read("*line")
        end

        temp = lines:sub(1, -2)
    end

    cprint("\n\n")

    return temp
end

local function string_to_table(string)
    local temptable = {}

    while string:find("\n") ~= nil do
        table.insert(temptable, string:sub(1, string:find("\n")-1))
        string = string:sub(string:find("\n") + 1)
    end

    if string ~= "" then
        table.insert(temptable, string)
    end

    return temptable
end

cprint("\n[Minetest Modpost Gen]\nPress enter twice with multiline input to move to the next question\n\n")

-- Get info needed

while info.techmodname == "" do
    cprint("Please enter the name for your mod as defined in your mod.conf")
    info.techmodname = getinput("line")
end

cprint("Please enter the user-friendly name for your mod (Skip to use capitalized mod.conf name)")
info.modname = getinput("line")

if info.modname == "" then info.modname = info.techmodname:sub(1, 1):upper() .. info.techmodname:sub(2) end

cprint("Please enter a short description (Skip for none)")
info.shortdescription = getinput("lines")

cprint("Do you have screenshots to provide? Answer yes/no")

repeat
    answer = getinput("line"):lower()
until answer:find("y") ~= nil or answer:find("n") ~= nil

if answer:find("y") ~= nil then -- answer was yes
    cprint("Please provide link(s) to your screenshot(s). One per line")
    info.screenshots = string_to_table(getinput("lines"))
end

cprint("Please enter the link to your Github repo (Skip for none)")
info.repository = getinput("line")

if info.repository ~= "" and info.repository:sub(-1, -1) ~= "/" then
    info.repository = info.repository .. "/"
end

cprint("Please enter the link to your CDB page")
info.contentdb = getinput("line")

if info.contentdb == "" then -- No CDB link provided, so can't create a download link out of it
    while info.download == "" do
        cprint("Please provide a download link")
        info.download = getinput("line")
    end
else
    if info.contentdb:sub(-1, -1) ~= "/" then
        info.contentdb = info.contentdb .. "/"
    end
end

while info.license_code == "" do
    cprint("Please enter the license of your code")
    info.license_code = getinput("line")
end

cprint("Please enter the license of your media (skip if your mod has no media)")
info.license_media = getinput("line")

cprint("Please enter the dependencies of your mod (skip for none)")
info.depends = getinput("line")

cprint("Please list the optional dependencies of your mod (skip for none)")
info.optional_depends = getinput("line")

if info.depends == "" then info.depends = "None" end
if info.optional_depends == "" then info.optional_depends = "None" end

-- Prepare info for generation

cprint("Is your mod WIP? Answer yes/no")

repeat
    answer = getinput("line"):lower()
until answer:find("y") ~= nil or answer:find("n") ~= nil

if answer:find("y") ~= nil then
    info.type = "WIP"
else
    info.type = "Mod"
end

cprint("Is your mod made only for a specific game? Type the name of it if so. Skip if not")
info.game = getinput("line")

if info.game ~= "" then
    cprint("Please provide a link to the game, if there is one")
    info.game_link = getinput("line")
end

--
--- Generate mod topic
--

local file = assert(io.open("output.txt", "w"))

-- Subject

file:write("Subject:\n")

file:write(("-\n[%s] %s [%s]\n-\n"):format(info.type, info.modname, info.techmodname))

-- Content

file:write("\nContent:\n-\n")

file:write(("[size=150][b]%s[/b][/size]\n\n"):format(info.modname))

if info.shortdescription ~= "" then
    file:write(info.shortdescription)
end

if info.screenshots ~= {} and #info.screenshots >= 1 then
    if #info.screenshots == 1 then
        file:write(("\n\n[img]%s[/img]"):format(info.screenshots[1]))
    else
        file:write("\n\n[spoiler=Screenshots]\n")
        
        for _, imglink in ipairs(info.screenshots) do
            file:write(("[img]%s[/img]\n"):format(imglink))
        end

        file:write("[/spoiler]")
    end
end

local download_link

if info.contentdb == "" then
    download_link = info.download
else
    download_link = info.contentdb .. "download/"
end

file:write(("\n\n[b]Downloads:[/b] [url=%s]Latest Stable[/url]\n\n"):format(download_link))

if info.contentdb ~= "" then
    file:write(("[b]View:[/b] [url=%s]On ContentDB[/url]\n"):format(info.contentdb))
end

if info.repository ~= "" then
    file:write(("[b]View:[/b] [url=%s]Source Code[/url]\n"):format(info.repository))
end

file:write("\n")

if info.license_media == "" then
    file:write(("[b]License:[/b] %s\n"):format(info.license_code))
else
    file:write(("[b]License of Code:[/b] %s\n"):format(info.license_code))
    file:write(("[b]License of Media:[/b] %s\n"):format(info.license_media))
end

if info.game == "" then
    file:write(("\n[b]Depends:[/b] %s\n"):format(info.depends))
    file:write(("[b]Optional Depends:[/b] %s"):format(info.optional_depends))
else
    if info.game_link == "" then
        file:write(("\n[b]Depends upon game:[/b] %s"):format(info.game))
    else
        file:write(("\n[b]Depends upon game:[/b] [url=%s]%s[/url]"):format(info.game_link, info.game))
    end
end

file:write("\n-\n")

file:close()

cprint("Modpost generated! If you want to include a full description put it at the end of the post.\n")
