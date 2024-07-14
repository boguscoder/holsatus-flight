use super::*;

pub const TX12_8CH_DEFAULT_MAP: TransmitterMap = TransmitterMap ([

    // Channel 0 - Right stick X
    ChannelType::Analog((
        AnalogCommand::Roll,
        AnalogConfig {
            in_min: 243,
            in_max: 1805,
            deadband: 2,
            fullrange: true,
            reverse: false,
            rates: Rates::new_standard(0.5, 20., 150.) 
        }
    )),

    // Channel 1 - Right stick Y
    ChannelType::Analog((
        AnalogCommand::Pitch,
        AnalogConfig {
            in_min: 243,
            in_max: 1805,
            deadband: 2,
            fullrange: true,
            reverse: true,
            rates: Rates::new_standard(0.5, 20., 150.) 
        }
    )),

    // Channel 2 - Left stick Y
    ChannelType::Analog((
        AnalogCommand::Thrust,
        AnalogConfig {
            in_min: 240,
            in_max: 1805,
            deadband: 2,
            fullrange: false,
            reverse: false,
            rates: Rates::None
        }
    )),

    // Channel 3 - Left stick X
    ChannelType::Analog((
        AnalogCommand::Yaw,
        AnalogConfig {
            in_min: 240,
            in_max: 1805,
            deadband: 2,
            fullrange: true,
            reverse: false,
            rates: Rates::new_standard(0.5, 20., 150.) 
        }
    )),
    
    // Channel 4 - Switch C
    ChannelType::Discrete([
        (240, EventRequest::DisarmMotors),
        (992, EventRequest::Unbound),
        (1750, EventRequest::ArmMotors),
    ]),
    // Channel 5 - Switch E
    ChannelType::Discrete([
        (240, EventRequest::Unbound),
        (992, EventRequest::AbortAccCalib),
        (1805, EventRequest::StartAccCalib),
    ]),
    // Channel 6 - Switch B
    ChannelType::Discrete([
        (240, EventRequest::AngleMode),
        (992, EventRequest::Unbound),
        (1805, EventRequest::RateMode),
        ]),


    // Channel 7 - Switch F
    ChannelType::Discrete([
        (172, EventRequest::Unbound),
        (992, EventRequest::Unbound),
        (1810, EventRequest::SaveConfig),
    ]),
    ChannelType::None,
    ChannelType::None,
    ChannelType::None,
    ChannelType::None,
    ChannelType::None,
    ChannelType::None,
    ChannelType::None,
    ChannelType::None,
]);

