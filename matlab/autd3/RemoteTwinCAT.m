%{
%File: RemoteTwinCAT.m
%Project: autd3
%Created Date: 10/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 08/08/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

classdef RemoteTwinCAT < handle

    properties
        ptr
        remote_ip_addr
        remote_ams_net_id 
        local_ams_net_id_
    end

    methods

        function obj = RemoteTwinCAT(remote_ip_addr, remote_ams_net_id)
            obj.ptr = libpointer('voidPtr', 0);
            obj.remote_ip_addr = remote_ip_addr;
            obj.remote_ams_net_id = remote_ams_net_id;
            obj.local_ams_net_id_ = "";
        end

        function local_ams_net_id(obj, id)
            obj.local_ams_net_id_ = id;
        end

        function res = build(obj)
            pp = libpointer('voidPtrPtr', obj.ptr);
            remote_ip_addr_ = libpointer('cstring', obj.remote_ip_addr);
            remote_ams_net_id_ = libpointer('cstring', obj.remote_ams_net_id);
            local_ams_net_id_p = libpointer('cstring', obj.local_ams_net_id_);
            calllib('autd3capi_link_remote_twincat', 'AUTDLinkRemoteTwinCAT', pp, remote_ip_addr_, remote_ams_net_id_, local_ams_net_id_p);
            res = obj;
        end

    end

end 
