%{
%File: deinit_autd.m
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

function deinit_autd(use_link_soem, use_backend_cuda)

    arguments
        use_link_soem = true
        use_backend_cuda = false
    end

    unloadlibrary('autd3capi');
    unloadlibrary('autd3capi_gain_holo');
    unloadlibrary('autd3capi_link_simulator');
    unloadlibrary('autd3capi_link_remote_twincat');
    unloadlibrary('autd3capi_modulation_audio_file');

    if ispc
        unloadlibrary('autd3capi_link_twincat');
    end

    if use_link_soem
        unloadlibrary('autd3capi_link_soem');
    end

    if use_backend_cuda
        unloadlibrary('autd3capi_backend_cuda');
    end

end
