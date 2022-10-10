# Mr Freeze

![mrfreeze](https://user-images.githubusercontent.com/62809/194907749-1c804952-2d68-4153-af1f-36667dac799a.png)

Minimal example to enable an aircon with the Daikin BRP072A4 Wi-Fi adapter.

Usage: `$ ./aircon HOST TEMPERATURE`

## Examples:

Set Aircon to 27 degrees Celsius (this is considered cold in Singapore): `$ ./aircon 192.168.0.3 27`

Set TEMPERATURE to any non-number to switch off: `./aircon 192.168.0.3 off`

You can use names instead of IP addresses by adding entries to /etc/hosts like this: `192.168.0.3 livingroom.local`

## Non-implemented commands

The Daikin Mobile Controller app includes calls to these API endpoints:

- /aircon/get_control_info
- /aircon/get_day_power_ex
- /aircon/get_demand_control
- /aircon/get_model_info
- /aircon/get_price
- /aircon/get_program
- /aircon/get_scdltimer
- /aircon/get_scdltimer_body
- /aircon/get_scdltimer_info
- /aircon/get_sensor_info
- /aircon/get_target
- /aircon/get_timer
- /aircon/get_week_power
- /aircon/get_week_power_ex
- /aircon/get_year_power
- /aircon/get_year_power_ex
- /aircon/set_control_info (the only one implemented with this app)
- /aircon/set_demand_control
- /aircon/set_price
- /aircon/set_program
- /aircon/set_scdltimer
- /aircon/set_scdltimer_body
- /aircon/set_scdltimer_info
- /aircon/set_special_mode
- /aircon/set_target
- /aircon/set_timer

This app only uses `set_control_info`.

## References

- https://github.com/ael-code/daikin-control/: Lists parameters and return values of the API endpoints
- https://play.google.com/store/apps/details?id=ao.daikin.remoapp: The APK that includes all API endpoint calls and parameters
- https://github.com/cbrandlehner/homebridge-daikin-local/: Homebridge plugin for Daikin Aircon to enable Apple HomeKit integration
