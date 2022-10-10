%{
%File: BackendCUDA.m
%Project: autd3
%Created Date: 10/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 10/06/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef BackendCUDA < Backend

    methods

        function obj = BackendCUDA()
            obj = obj@Backend();
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi_backend_cuda', 'AUTDCUDABackend', pp);
        end

    end

end
