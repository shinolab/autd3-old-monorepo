function get_os_info()
    if Sys.iswindows()
        return "win-x64"
    elseif Sys.isapple()
        return "macos-universal"
    elseif Sys.islinux()
        return "linux-x64"
    else
        error("This OS is not supported.")
    end
end

function get_archive_file_ext()
    if Sys.iswindows()
        return ".zip"
    else
        return ".tar.gz"
    end
end

function get_lib_ext()
    if Sys.iswindows()
        return ".dll"
    elseif Sys.isapple()
        return ".dylib"
    elseif Sys.islinux()
        return ".so"
    end
end

function get_version()
    proj_toml_path = joinpath(@__DIR__, "..", "Project.toml")
    open(proj_toml_path, "r") do fp
        for line in eachline(fp)
            println(line)
            if startswith(line, "version = ")
                return string(strip(replace(line, "version = " => ""), '\"'))
            end
        end
        error("Version is not found in Project.toml.")
    end
end

function extract_zip(zipfile)
    lib_ext = get_lib_ext()
    run(`powershell Expand-Archive -Path $zipfile`)
    cp("tmp/bin", joinpath(@__DIR__, "..", "src", "NativeMethods", "bin", get_os_info()); force=true)
    rm("tmp"; force=true, recursive=true)
    rm(zipfile; force=true)
end

function extract_targz(tarfile)
    run(`tar -xvf $tarfile`)
    cp("bin", joinpath(@__DIR__, "..", "src", "NativeMethods", "bin", get_os_info()); force=true)
    rm("bin"; force=true, recursive=true)
    rm(tarfile; force=true)
end

function replace_latest_binary(version::String)
    local base_url = "https://github.com/shinolab/autd3/releases/download/"
    module_path = joinpath(@__DIR__, "..", "src")
    ext = get_archive_file_ext()
    os_info = get_os_info()
    url = base_url * "/v" * version * "/autd3-" * "v" * version * "-" * os_info * ext
    tmp_archive_path = joinpath(module_path, "tmp" * ext)
    download(url, tmp_archive_path)
    if startswith(os_info, "win")
        extract_zip(tmp_archive_path)
    elseif startswith(os_info, "macos")
        extract_targz(tmp_archive_path)
    elseif startswith(os_info, "linux")
        extract_targz(tmp_archive_path)
    end
end

try
    _version = get_version()
    replace_latest_binary(_version)
catch
    println("Cannot download", get_version(), "binaries...")
end
