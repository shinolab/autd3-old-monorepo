%{
%File: Uniform.m
%Project: autd3
%Created Date: 10/11/2022
%Author: Shun Suzuki
%-----
%Last Modified: 10/11/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef Uniform < Constraint

    methods

        function obj = Uniform(value)
            obj = obj@Constraint();
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi_gain_holo', 'AUTDConstraintUniform', pp, value);

        end

    end

end
