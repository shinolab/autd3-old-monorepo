%{
%File: init_autd.m
%Project: autd3-matlab
%Created Date: 07/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 10/10/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

function init_autd(use_link_soem, use_backend_cuda)

    arguments
        use_link_soem = true
        use_backend_cuda = false
    end

    cd('bin');

    if ispc

        loadlibrary('win-x64/autd3capi.dll');
        loadlibrary('win-x64/autd3capi_gain_holo.dll');

        if use_link_soem
            loadlibrary('win-x64/autd3capi_link_soem.dll');
        end

        loadlibrary('win-x64/autd3capi_link_simulator.dll');
        loadlibrary('win-x64/autd3capi_link_remote_twincat.dll');
        loadlibrary('win-x64/autd3capi_link_twincat.dll');

        if use_backend_cuda
            loadlibrary('win-x64/autd3capi_backend_cuda.dll');
        end

        loadlibrary('win-x64/autd3capi_modulation_audio_file.dll');

    elseif ismac

        loadlibrary('macos-universal/autd3capi.dylib', 'autd3capi.h');
        loadlibrary('macos-universal/autd3capi_gain_holo.dylib', 'autd3capi_gain_holo.h');

        if use_link_soem
            loadlibrary('macos-universal/autd3capi_link_soem.dylib', 'autd3capi_link_soem.h');
        end

        loadlibrary('macos-universal/autd3capi_link_simulator.dylib', 'autd3capi_link_simulator.h');
        loadlibrary('macos-universal/autd3capi_link_remote_twincat.dylib', 'autd3capi_link_remote_twincat.h');

        loadlibrary('macos-universal/autd3capi_modulation_audio_file.dylib', 'autd3capi_modulation_audio_file.h');

    elseif isunix

        loadlibrary('linux-x64/autd3capi.so', 'autd3capi.h');
        loadlibrary('linux-x64/autd3capi_gain_holo.so', 'autd3capi_gain_holo.h');

        if use_link_soem
            loadlibrary('linux-x64/autd3capi_link_soem.so', 'autd3capi_link_soem.h');
        end

        if use_backend_cuda
            loadlibrary('linux-x64/autd3capi_backend_cuda.so', 'autd3capi_backend_cuda.h');
        end

        loadlibrary('linux-x64/autd3capi_link_simulator.so', 'autd3capi_link_simulator.h');
        loadlibrary('linux-x64/autd3capi_link_remote_twincat.so', 'autd3capi_link_remote_twincat.h');

        loadlibrary('linux-x64/autd3capi_modulation_audio_file.so', 'autd3capi_modulation_audio_file.h');

    else
        disp('Platform not supported');
        return;
    end

    cd('..');

end
