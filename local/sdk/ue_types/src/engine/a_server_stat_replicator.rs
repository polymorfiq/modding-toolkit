use crate::*;
use simple_endian::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct AServerStatReplicator {
    pub base_ainfo: AInfo,
    pub b_update_stat_net: u8,
    pub b_overwrite_client_stats: u8,
    _unknown_a: [u8; 0x2],
    pub channels: u32le,
    pub in_rate: u32le,
    pub out_rate: u32le,
    pub out_saturation: u32le,
    pub max_packet_overhead: u32le,
    pub in_rate_client_max: u32le,
    pub in_rate_client_min: u32le,
    pub in_rate_client_avg: u32le,
    pub in_packets_client_max: u32le,
    pub in_packets_client_min: u32le,
    pub in_packets_client_avg: u32le,
    pub out_rate_client_max: u32le,
    pub out_rate_client_min: u32le,
    pub out_rate_client_avg: u32le,
    pub out_packets_client_max: u32le,
    pub out_packets_client_min: u32le,
    pub out_packets_client_avg: u32le,
    pub net_num_clients: u32le,
    pub in_packets: u32le,
    pub out_packets: u32le,
    pub in_bunches: u32le,
    pub out_bunches: u32le,
    pub out_loss: u32le,
    pub in_loss: u32le,
    pub voice_bytes_sent: u32le,
    pub voice_bytes_recv: u32le,
    pub voice_packets_sent: u32le,
    pub voice_packets_recv: u32le,
    pub percent_in_voice: u32le,
    pub percent_out_voice: u32le,
    pub num_actor_channels: u32le,
    pub num_considered_actors: u32le,
    pub prioritized_actors: u32le,
    pub num_relevant_actors: u32le,
    pub num_relevant_deleted_actors: u32le,
    pub num_replicated_actor_attempts: u32le,
    pub num_replicated_actors: u32le,
    pub num_actors: u32le,
    pub num_net_actors: u32le,
    pub num_dormant_actors: u32le,
    pub num_initially_dormant_actors: u32le,
    pub num_net_guids_ackd: u32le,
    pub num_net_guids_pending: u32le,
    pub num_net_guids_unacked: u32le,
    pub obj_path_bytes: u32le,
    pub net_guid_out_rate: u32le,
    pub net_guid_in_rate: u32le,
    pub net_saturated: u32le,
    _unknown_b: [u8; 0x4]
}