


struct Signal<Wave> 
where 
    Wave: Waveform,
{
    signal_id: u32,
    components: Vec<Wave>, 
    num_points: u64,
}

pub trait Waveform {

    


}