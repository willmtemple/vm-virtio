use crate::FuzzingDescriptor;
use virtio_vsock::packet::VsockPacket;

use serde::{Deserialize, Serialize};
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

/// All the functions that can be called when fuzzing the VsockPacket.
#[derive(Serialize, Deserialize, Debug)]
pub enum VsockFunction {
    HeaderSlice,
    Len,
    DataSlice,
    SrcCid,
    SetSrcCid { cid: u64 },
    DstCid,
    SetDstCid { cid: u64 },
    SrcPort,
    SetSrcPort { port: u32 },
    DstPort,
    SetDstPort { port: u32 },
    IsEmpty,
    SetLen { len: u32 },
    Type_,
    SetType { type_: u16 },
    Op,
    SetOp { op: u16 },
    Flags,
    SetFlags { flags: u32 },
    SetFlag { flag: u32 },
    BufAlloc,
    SetBufAlloc { buf_alloc: u32 },
    FwdCnt,
    SetFwdCnt { fwd_cnt: u32 },
    SetHeaderFromRaw { bytes: Vec<u8> },
    // This function is not part of the VsockPacket interface but it is needed
    // to be able to read non-zero bytes from Guest Memory.
    // We're using this to generate custom input for the fuzzer by writing a
    // VsockPacket to Guest Memory that is later going to be read and altered
    // during fuzzing.
    _WriteToMem { addr: u64, bytes: Vec<u8> },
}

impl VsockFunction {
    pub fn call<B: vm_memory::bitmap::BitmapSlice>(
        &self,
        packet: &mut VsockPacket<B>,
        mem: &mut GuestMemoryMmap,
    ) {
        match self {
            VsockFunction::HeaderSlice => {
                packet.header_slice();
            }
            VsockFunction::Len => {
                packet.len();
            }
            VsockFunction::DataSlice => {
                packet.data_slice();
            }
            VsockFunction::SrcCid => {
                packet.src_cid();
            }
            VsockFunction::SetSrcCid { cid } => {
                packet.set_src_cid(*cid);
            }
            VsockFunction::DstCid => {
                packet.dst_cid();
            }
            VsockFunction::SetDstCid { cid } => {
                packet.set_dst_cid(*cid);
            }
            VsockFunction::SrcPort => {
                packet.src_port();
            }
            VsockFunction::SetSrcPort { port } => {
                packet.set_src_port(*port);
            }
            VsockFunction::DstPort => {
                packet.dst_port();
            }
            VsockFunction::SetDstPort { port } => {
                packet.set_dst_port(*port);
            }
            VsockFunction::IsEmpty => {
                packet.is_empty();
            }
            VsockFunction::SetLen { len } => {
                packet.set_len(*len);
            }
            VsockFunction::Type_ => {
                packet.type_();
            }
            VsockFunction::SetType { type_ } => {
                packet.set_type(*type_);
            }
            VsockFunction::Op => {
                packet.op();
            }
            VsockFunction::SetOp { op } => {
                packet.set_op(*op);
            }
            VsockFunction::Flags => {
                packet.flags();
            }
            VsockFunction::SetFlags { flags } => {
                packet.set_flags(*flags);
            }
            VsockFunction::SetFlag { flag } => {
                packet.set_flag(*flag);
            }
            VsockFunction::BufAlloc => {
                packet.buf_alloc();
            }
            VsockFunction::SetBufAlloc { buf_alloc } => {
                packet.set_buf_alloc(*buf_alloc);
            }
            VsockFunction::FwdCnt => {
                packet.fwd_cnt();
            }
            VsockFunction::SetFwdCnt { fwd_cnt } => {
                packet.set_fwd_cnt(*fwd_cnt);
            }
            VsockFunction::SetHeaderFromRaw { bytes } => {
                let _ = packet.set_header_from_raw(bytes.as_slice());
            }
            VsockFunction::_WriteToMem { bytes, addr } => {
                let _ = mem.write_slice(bytes.as_slice(), GuestAddress(*addr));
            }
        }
    }
}

// Whether we create a VsockPacket from_rx_virtq_chain or from_tx_virtq_chain
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum InitFunction {
    FromRX,
    FromTX,
}

/// Input generated by the fuzzer for fuzzing vsock_rx and vsock_tx
#[derive(Serialize, Deserialize, Debug)]
pub struct VsockInput {
    pub pkt_max_data: u32,
    pub init_function: InitFunction,
    pub descriptors: Vec<FuzzingDescriptor>,
    pub functions: Vec<VsockFunction>,
}
