use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ForecastMeta {
    pub units: ForecastUnits,
    #[doc = " Update time for this forecast"]
    pub updated_at: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Forecast {
    pub meta: ForecastMeta,
    pub timeseries: Vec<ForecastTimeStep>,
}
#[doc = " Summary of weather conditions."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ForecastSummary {
    pub symbol_code: WeatherSymbol,
}
#[doc = " Weather parameters valid for a specific point in time."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ForecastTimeInstant {
    #[doc = " Air pressure at sea level"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_pressure_at_sea_level: Option<f64>,
    #[doc = " Air temperature"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_temperature: Option<f64>,
    #[doc = " Amount of sky covered by clouds."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction: Option<f64>,
    #[doc = " Amount of sky covered by clouds at high elevation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction_high: Option<f64>,
    #[doc = " Amount of sky covered by clouds at low elevation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction_low: Option<f64>,
    #[doc = " Amount of sky covered by clouds at medium elevation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction_medium: Option<f64>,
    #[doc = " Dew point temperature at sea level"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dew_point_temperature: Option<f64>,
    #[doc = " Amount of area covered by fog."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fog_area_fraction: Option<f64>,
    #[doc = " Amount of humidity in the air."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_humidity: Option<f64>,
    #[doc = " The directon which moves towards"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_from_direction: Option<f64>,
    #[doc = " Speed of wind"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_speed: Option<f64>,
    #[doc = " Speed of wind gust"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_speed_of_gust: Option<f64>,
}
#[doc = " Weather parameters valid for a specified time period."]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ForecastTimePeriod {
    #[doc = " Maximum air temperature in period"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_temperature_max: Option<f64>,
    #[doc = " Minimum air temperature in period"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_temperature_min: Option<f64>,
    #[doc = " Best estimate for amount of precipitation for this period"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_amount: Option<f64>,
    #[doc = " Maximum amount of precipitation for this period"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_amount_max: Option<f64>,
    #[doc = " Minimum amount of precipitation for this period"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_amount_min: Option<f64>,
    #[doc = " Probability of any precipitation coming for this period"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability_of_precipitation: Option<f64>,
    #[doc = " Probability of any thunder coming for this period"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability_of_thunder: Option<f64>,
    #[doc = " Maximum ultraviolet index if sky is clear"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultraviolet_index_clear_sky_max: Option<f64>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ForecastTimeStepDataInstant {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ForecastTimeInstant>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ForecastTimeStepDataNext12Hours {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ForecastTimePeriod>,
    pub summary: ForecastSummary,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ForecastTimeStepDataNext1Hours {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ForecastTimePeriod>,
    pub summary: ForecastSummary,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ForecastTimeStepDataNext6Hours {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ForecastTimePeriod>,
    pub summary: ForecastSummary,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ForecastTimeStepData {
    #[doc = " Parameters which applies to this exact point in time"]
    pub instant: ForecastTimeStepDataInstant,
    #[doc = " Parameters with validity times over twelve hours. Will not exist for all time steps."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_12_hours: Option<ForecastTimeStepDataNext12Hours>,
    #[doc = " Parameters with validity times over one hour. Will not exist for all time steps."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_1_hours: Option<ForecastTimeStepDataNext1Hours>,
    #[doc = " Parameters with validity times over six hours. Will not exist for all time steps."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_6_hours: Option<ForecastTimeStepDataNext6Hours>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ForecastTimeStep {
    #[doc = " Forecast for a specific time"]
    pub data: ForecastTimeStepData,
    #[doc = " The time these forecast values are valid for. Timestamp in format YYYY-MM-DDThh:mm:ssZ (ISO "]
    #[doc = " 8601)"]
    pub time: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ForecastUnits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_pressure_at_sea_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_temperature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_temperature_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_temperature_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction_high: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction_low: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction_medium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dew_point_temperature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fog_area_fraction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_amount_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_amount_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability_of_precipitation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability_of_thunder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_humidity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultraviolet_index_clear_sky_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_from_direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_speed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_speed_of_gust: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "METJSONForecast")]
pub struct Metjsonforecast {
    pub geometry: PointGeometry,
    pub properties: Forecast,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct PointGeometry {
    #[doc = " [longitude, latitude, altitude]. All numbers in decimal."]
    pub coordinates: Vec<f64>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[doc = " A identifier that sums up the weather condition for this time period, see documentation."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum WeatherSymbol {
    #[serde(rename = "clearsky_day")]
    ClearskyDay,
    #[serde(rename = "clearsky_night")]
    ClearskyNight,
    #[serde(rename = "clearsky_polartwilight")]
    ClearskyPolartwilight,
    #[serde(rename = "fair_day")]
    FairDay,
    #[serde(rename = "fair_night")]
    FairNight,
    #[serde(rename = "fair_polartwilight")]
    FairPolartwilight,
    #[serde(rename = "lightssnowshowersandthunder_day")]
    LightssnowshowersandthunderDay,
    #[serde(rename = "lightssnowshowersandthunder_night")]
    LightssnowshowersandthunderNight,
    #[serde(rename = "lightssnowshowersandthunder_polartwilight")]
    LightssnowshowersandthunderPolartwilight,
    #[serde(rename = "lightsnowshowers_day")]
    LightsnowshowersDay,
    #[serde(rename = "lightsnowshowers_night")]
    LightsnowshowersNight,
    #[serde(rename = "lightsnowshowers_polartwilight")]
    LightsnowshowersPolartwilight,
    #[serde(rename = "heavyrainandthunder")]
    Heavyrainandthunder,
    #[serde(rename = "heavysnowandthunder")]
    Heavysnowandthunder,
    #[serde(rename = "rainandthunder")]
    Rainandthunder,
    #[serde(rename = "heavysleetshowersandthunder_day")]
    HeavysleetshowersandthunderDay,
    #[serde(rename = "heavysleetshowersandthunder_night")]
    HeavysleetshowersandthunderNight,
    #[serde(rename = "heavysleetshowersandthunder_polartwilight")]
    HeavysleetshowersandthunderPolartwilight,
    #[serde(rename = "heavysnow")]
    Heavysnow,
    #[serde(rename = "heavyrainshowers_day")]
    HeavyrainshowersDay,
    #[serde(rename = "heavyrainshowers_night")]
    HeavyrainshowersNight,
    #[serde(rename = "heavyrainshowers_polartwilight")]
    HeavyrainshowersPolartwilight,
    #[serde(rename = "lightsleet")]
    Lightsleet,
    #[serde(rename = "heavyrain")]
    Heavyrain,
    #[serde(rename = "lightrainshowers_day")]
    LightrainshowersDay,
    #[serde(rename = "lightrainshowers_night")]
    LightrainshowersNight,
    #[serde(rename = "lightrainshowers_polartwilight")]
    LightrainshowersPolartwilight,
    #[serde(rename = "heavysleetshowers_day")]
    HeavysleetshowersDay,
    #[serde(rename = "heavysleetshowers_night")]
    HeavysleetshowersNight,
    #[serde(rename = "heavysleetshowers_polartwilight")]
    HeavysleetshowersPolartwilight,
    #[serde(rename = "lightsleetshowers_day")]
    LightsleetshowersDay,
    #[serde(rename = "lightsleetshowers_night")]
    LightsleetshowersNight,
    #[serde(rename = "lightsleetshowers_polartwilight")]
    LightsleetshowersPolartwilight,
    #[serde(rename = "snow")]
    Snow,
    #[serde(rename = "heavyrainshowersandthunder_day")]
    HeavyrainshowersandthunderDay,
    #[serde(rename = "heavyrainshowersandthunder_night")]
    HeavyrainshowersandthunderNight,
    #[serde(rename = "heavyrainshowersandthunder_polartwilight")]
    HeavyrainshowersandthunderPolartwilight,
    #[serde(rename = "snowshowers_day")]
    SnowshowersDay,
    #[serde(rename = "snowshowers_night")]
    SnowshowersNight,
    #[serde(rename = "snowshowers_polartwilight")]
    SnowshowersPolartwilight,
    #[serde(rename = "fog")]
    Fog,
    #[serde(rename = "snowshowersandthunder_day")]
    SnowshowersandthunderDay,
    #[serde(rename = "snowshowersandthunder_night")]
    SnowshowersandthunderNight,
    #[serde(rename = "snowshowersandthunder_polartwilight")]
    SnowshowersandthunderPolartwilight,
    #[serde(rename = "lightsnowandthunder")]
    Lightsnowandthunder,
    #[serde(rename = "heavysleetandthunder")]
    Heavysleetandthunder,
    #[serde(rename = "lightrain")]
    Lightrain,
    #[serde(rename = "rainshowersandthunder_day")]
    RainshowersandthunderDay,
    #[serde(rename = "rainshowersandthunder_night")]
    RainshowersandthunderNight,
    #[serde(rename = "rainshowersandthunder_polartwilight")]
    RainshowersandthunderPolartwilight,
    #[serde(rename = "rain")]
    Rain,
    #[serde(rename = "lightsnow")]
    Lightsnow,
    #[serde(rename = "lightrainshowersandthunder_day")]
    LightrainshowersandthunderDay,
    #[serde(rename = "lightrainshowersandthunder_night")]
    LightrainshowersandthunderNight,
    #[serde(rename = "lightrainshowersandthunder_polartwilight")]
    LightrainshowersandthunderPolartwilight,
    #[serde(rename = "heavysleet")]
    Heavysleet,
    #[serde(rename = "sleetandthunder")]
    Sleetandthunder,
    #[serde(rename = "lightrainandthunder")]
    Lightrainandthunder,
    #[serde(rename = "sleet")]
    Sleet,
    #[serde(rename = "lightssleetshowersandthunder_day")]
    LightssleetshowersandthunderDay,
    #[serde(rename = "lightssleetshowersandthunder_night")]
    LightssleetshowersandthunderNight,
    #[serde(rename = "lightssleetshowersandthunder_polartwilight")]
    LightssleetshowersandthunderPolartwilight,
    #[serde(rename = "lightsleetandthunder")]
    Lightsleetandthunder,
    #[serde(rename = "partlycloudy_day")]
    PartlycloudyDay,
    #[serde(rename = "partlycloudy_night")]
    PartlycloudyNight,
    #[serde(rename = "partlycloudy_polartwilight")]
    PartlycloudyPolartwilight,
    #[serde(rename = "sleetshowersandthunder_day")]
    SleetshowersandthunderDay,
    #[serde(rename = "sleetshowersandthunder_night")]
    SleetshowersandthunderNight,
    #[serde(rename = "sleetshowersandthunder_polartwilight")]
    SleetshowersandthunderPolartwilight,
    #[serde(rename = "rainshowers_day")]
    RainshowersDay,
    #[serde(rename = "rainshowers_night")]
    RainshowersNight,
    #[serde(rename = "rainshowers_polartwilight")]
    RainshowersPolartwilight,
    #[serde(rename = "snowandthunder")]
    Snowandthunder,
    #[serde(rename = "sleetshowers_day")]
    SleetshowersDay,
    #[serde(rename = "sleetshowers_night")]
    SleetshowersNight,
    #[serde(rename = "sleetshowers_polartwilight")]
    SleetshowersPolartwilight,
    #[serde(rename = "cloudy")]
    Cloudy,
    #[serde(rename = "heavysnowshowersandthunder_day")]
    HeavysnowshowersandthunderDay,
    #[serde(rename = "heavysnowshowersandthunder_night")]
    HeavysnowshowersandthunderNight,
    #[serde(rename = "heavysnowshowersandthunder_polartwilight")]
    HeavysnowshowersandthunderPolartwilight,
    #[serde(rename = "heavysnowshowers_day")]
    HeavysnowshowersDay,
    #[serde(rename = "heavysnowshowers_night")]
    HeavysnowshowersNight,
    #[serde(rename = "heavysnowshowers_polartwilight")]
    HeavysnowshowersPolartwilight,
}
pub type Schema = serde_json::Value;
