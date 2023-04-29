%{
%File: RemoteTwinCAT.m
%Project: autd3
%Created Date: 10/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 28/04/2023
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef RemoteTwinCAT < handle

    properties
        ptr
        builder_
    end

    methods

        function obj = RemoteTwinCAT(remote_ams_net_id)
            obj.ptr = libpointer('voidPtr', 0);
            obj.builder_ = libpointer('voidPtr', 0);
            pp = libpointer('voidPtrPtr', obj.builder_);
            calllib('autd3capi_link_remote_twincat', 'AUTDLinkRemoteTwinCAT', pp, remote_ams_net_id);
        end

        function server_ip(obj, ip)
            calllib('autd3capi_link_remote_twincat', 'AUTDLinkRemoteTwinCATServerIpAddr', obj.builder_, ip);
        end

        function client_ams_net_id(obj, id)
            calllib('autd3capi_link_remote_twincat', 'AUTDLinkRemoteTwinCATClientAmsNetId', obj.builder_, id);
        end

        function res = timeout(obj, t)
            calllib('autd3capi_link_remote_twincat', 'AUTDLinkRemoteTwinCATTimeout', obj.builder_, t);
            res = obj;
        end
        
        function res = build(obj)
            pp = libpointer('voidPtrPtr', obj.ptr);
            calllib('autd3capi_link_remote_twincat', 'AUTDLinkRemoteTwinCATBuild', pp, obj.builder_);
            res = obj;
        end

    end

end 
