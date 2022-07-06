/// Encapsulates an encoded polyline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Polyline {
    /// Encapsulates the type of polyline. Defaults to encoded_polyline.
    #[prost(oneof="polyline::PolylineType", tags="1, 2")]
    pub polyline_type: ::core::option::Option<polyline::PolylineType>,
}
/// Nested message and enum types in `Polyline`.
pub mod polyline {
    /// Encapsulates the type of polyline. Defaults to encoded_polyline.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PolylineType {
        /// The string encoding of the polyline using the [polyline encoding
        /// algorithm](<https://developers.google.com/maps/documentation/utilities/polylinealgorithm>)
        #[prost(string, tag="1")]
        EncodedPolyline(::prost::alloc::string::String),
        /// Specifies a polyline using the [GeoJSON LineString
        /// format](<https://tools.ietf.org/html/rfc7946#section-3.1.4>)
        #[prost(message, tag="2")]
        GeoJsonLinestring(::prost_types::Struct),
    }
}
/// A set of values that specify the quality of the polyline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PolylineQuality {
    /// No polyline quality preference specified. Defaults to `OVERVIEW`.
    Unspecified = 0,
    /// Specifies a high-quality polyline - which is composed using more points
    /// than `OVERVIEW`, at the cost of increased response size. Use this value
    /// when you need more precision.
    HighQuality = 1,
    /// Specifies an overview polyline - which is composed using a small number of
    /// points. Use this value when displaying an overview of the route. Using this
    /// option has a lower request latency compared to using the
    /// `HIGH_QUALITY` option.
    Overview = 2,
}
/// Specifies the preferred type of polyline to be returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PolylineEncoding {
    /// No polyline type preference specified. Defaults to `ENCODED_POLYLINE`.
    Unspecified = 0,
    /// Specifies a polyline encoded using the [polyline encoding
    /// algorithm](<https://developers.google.com/maps/documentation/utilities/polylinealgorithm>).
    EncodedPolyline = 1,
    /// Specifies a polyline using the [GeoJSON LineString
    /// format](<https://tools.ietf.org/html/rfc7946#section-3.1.4>)
    GeoJsonLinestring = 2,
}
/// List of toll passes around the world that we support.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TollPass {
    /// Not used. If this value is used, then the request fails.
    Unspecified = 0,
    /// Australia-wide toll pass.
    /// See additional details at <https://www.linkt.com.au/.>
    AuLinkt = 2,
    /// Argentina toll pass. See additional details at <https://telepase.com.ar>
    ArTelepase = 3,
    /// Brazil toll pass. See additional details at <https://conectcar.com.>
    BrConectcar = 7,
    /// Brazil toll pass. See additional details at <https://movemais.com.>
    BrMoveMais = 8,
    /// Brazil toll pass. See additional details at <https://www.semparar.com.br.>
    BrSemParar = 9,
    /// Brazil toll pass. See additional details at <https://taggy.com.br.>
    BrTaggy = 10,
    /// Brazil toll pass. See additional details at
    /// <https://veloe.com.br/site/onde-usar.>
    BrVeloe = 11,
    /// Indonesia.
    /// E-card provided by multiple banks used to pay for tolls. All e-cards
    /// via banks are charged the same so only one enum value is needed. E.g.
    /// Bank Mandiri <https://www.bankmandiri.co.id/e-money>
    /// BCA <https://www.bca.co.id/flazz>
    /// BNI <https://www.bni.co.id/id-id/ebanking/tapcash>
    IdEToll = 16,
    /// Mexico toll pass.
    MxTagIave = 12,
    /// Mexico toll pass company. One of many operating in Mexico City. See
    /// additional details at <https://www.televia.com.mx.>
    MxTagTelevia = 13,
    /// Mexico toll pass. See additional details at
    /// <https://www.viapass.com.mx/viapass/web_home.aspx.>
    MxViapass = 14,
    /// State pass of California, United States. Passes vary between Standard,
    /// Flex, and Clean Air. Flex and Clean Air have settings for carpool. See
    /// additional details at <https://www.bayareafastrak.org/en/home/index.shtml.>
    UsCaFastrak = 4,
    /// State pass of Illinois, United States. See additional details at
    /// <https://www.illinoistollway.com/about-ipass.>
    UsIlIpass = 5,
    /// State pass of Massachusetts, United States. See additional details at
    /// <https://www.mass.gov/ezdrivema.>
    UsMaEzpassma = 6,
    /// State pass of New York, United States. See additional details at
    /// <https://www.e-zpassny.com.>
    UsNyEzpassny = 15,
    /// State pass of the Washington state, United States.
    UsWaGoodToGo = 1,
}
/// A set of values describing the vehicle's emission type.
/// Applies only to the DRIVE travel mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VehicleEmissionType {
    /// No emission type specified. Default to GASOLINE.
    Unspecified = 0,
    /// Gasoline/petrol fueled vehicle.
    Gasoline = 1,
    /// Electricity powered vehicle.
    Electric = 2,
    /// Hybrid fuel (such as gasoline + electric) vehicle.
    Hybrid = 3,
}
/// Encapsulates a waypoint. Waypoints mark both the beginning and end of a
/// route, and include intermediate stops along the route.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Waypoint {
    /// Marks this waypoint as a milestone, as opposed to a stopping point. For
    /// each non-via waypoint in the request, the response appends an entry to the
    /// `legs` array to provide the details for stopovers on that leg of the
    /// trip. Set this value to true when you want the route to pass through this
    /// waypoint without stopping over. Via waypoints don't cause an entry to be
    /// added to the `legs` array, but they do route the journey through the
    /// waypoint. You can only set this value on waypoints that are intermediates.
    /// If you set this field on terminal waypoints, then the request fails.
    #[prost(bool, tag="3")]
    pub via: bool,
    /// Indicates that the waypoint is meant for vehicles to stop at, where the
    /// intention is to either pickup or drop-off. When you set this value, the
    /// calculated route won't include non-`via` waypoints on roads that are
    /// unsuitable for pickup and drop-off. This option works only for `DRIVE` and
    /// `TWO_WHEELER` travel modes, and when the `location_type` is `location`.
    #[prost(bool, tag="4")]
    pub vehicle_stopover: bool,
    /// Indicates that the location of this waypoint is meant to have a preference
    /// for the vehicle to stop at a particular side of road. When you set this
    /// value, the route will pass through the location so that the vehicle can
    /// stop at the side of road that the location is biased towards from the
    /// center of the road. This option works only for 'DRIVE' and 'TWO_WHEELER'
    /// travel modes, and when the 'location_type' is set to 'location'.
    #[prost(bool, tag="5")]
    pub side_of_road: bool,
    /// Different ways to represent a location.
    #[prost(oneof="waypoint::LocationType", tags="1, 2")]
    pub location_type: ::core::option::Option<waypoint::LocationType>,
}
/// Nested message and enum types in `Waypoint`.
pub mod waypoint {
    /// Different ways to represent a location.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LocationType {
        /// A point specified using geographic coordinates, including an optional
        /// heading.
        #[prost(message, tag="1")]
        Location(super::Location),
        /// The POI Place ID associated with the waypoint.
        #[prost(string, tag="2")]
        PlaceId(::prost::alloc::string::String),
    }
}
/// Encapsulates a location (a geographic point, and an optional heading).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// The waypoint's geographic coordinates.
    #[prost(message, optional, tag="1")]
    pub lat_lng: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The compass heading associated with the direction of the flow of traffic.
    /// This value is used to specify the side of the road to use for pickup and
    /// drop-off. Heading values can be from 0 to 360, where 0 specifies a heading
    /// of due North, 90 specifies a heading of due East, etc. You can use this
    /// field only for `DRIVE` and `TWO_WHEELER` travel modes.
    #[prost(message, optional, tag="2")]
    pub heading: ::core::option::Option<i32>,
}
/// ComputeRoutes request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeRoutesRequest {
    /// Required. Origin waypoint.
    #[prost(message, optional, tag="1")]
    pub origin: ::core::option::Option<Waypoint>,
    /// Required. Destination waypoint.
    #[prost(message, optional, tag="2")]
    pub destination: ::core::option::Option<Waypoint>,
    /// Optional. A set of waypoints along the route (excluding terminal points),
    /// for either stopping at or passing by. Up to 25 intermediate waypoints are
    /// supported.
    #[prost(message, repeated, tag="3")]
    pub intermediates: ::prost::alloc::vec::Vec<Waypoint>,
    /// Optional. Specifies the mode of transportation.
    #[prost(enumeration="RouteTravelMode", tag="4")]
    pub travel_mode: i32,
    /// Optional. Specifies how to compute the route. The server
    /// attempts to use the selected routing preference to compute the route. If
    ///  the routing preference results in an error or an extra long latency, then
    /// an error is returned. In the future, we might implement a fallback
    /// mechanism to use a different option when the preferred option does not give
    /// a valid result. You can specify this option only when the `travel_mode` is
    /// `DRIVE` or `TWO_WHEELER`, otherwise the request fails.
    #[prost(enumeration="RoutingPreference", tag="5")]
    pub routing_preference: i32,
    /// Optional. Specifies your preference for the quality of the polyline.
    #[prost(enumeration="PolylineQuality", tag="6")]
    pub polyline_quality: i32,
    /// Optional. Specifies the preferred encoding for the polyline.
    #[prost(enumeration="PolylineEncoding", tag="12")]
    pub polyline_encoding: i32,
    /// Optional. The departure time. If you don't set this value, then this value
    /// defaults to the time that you made the request. If you set this value to a
    /// time that has already occurred, then the request fails.
    #[prost(message, optional, tag="7")]
    pub departure_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Specifies whether to calculate alternate routes in addition to the route.
    #[prost(bool, tag="8")]
    pub compute_alternative_routes: bool,
    /// Optional. A set of conditions to satisfy that affect the way routes are
    /// calculated.
    #[prost(message, optional, tag="9")]
    pub route_modifiers: ::core::option::Option<RouteModifiers>,
    /// Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.> See
    /// [Language Support](<https://developers.google.com/maps/faq#languagesupport>)
    /// for the list of supported languages. When you don't provide this value, the
    /// display language is inferred from the location of the route request.
    #[prost(string, tag="10")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. Specifies the units of measure for the display fields. This
    /// includes the `instruction` field in `NavigationInstruction`. The units of
    /// measure used for the route, leg, step distance, and duration are not
    /// affected by this value. If you don't provide this value, then the display
    /// units are inferred from the location of the request.
    #[prost(enumeration="Units", tag="11")]
    pub units: i32,
}
/// Encapsulates a set of optional conditions to satisfy when calculating the
/// routes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteModifiers {
    /// Specifies whether to avoid toll roads where reasonable. Preference will be
    /// given to routes not containing toll roads. Applies only to the `DRIVE` and
    /// `TWO_WHEELER` travel modes.
    #[prost(bool, tag="1")]
    pub avoid_tolls: bool,
    /// Specifies whether to avoid highways where reasonable. Preference will be
    /// given to routes not containing highways. Applies only to the `DRIVE` and
    /// `TWO_WHEELER` travel modes.
    #[prost(bool, tag="2")]
    pub avoid_highways: bool,
    /// Specifies whether to avoid ferries where reasonable. Preference will be
    /// given to routes not containing travel by ferries.
    /// Applies only to the `DRIVE` and`TWO_WHEELER` travel modes.
    #[prost(bool, tag="3")]
    pub avoid_ferries: bool,
    /// Specifies whether to avoid navigating indoors where reasonable. Preference
    /// will be given to routes not containing indoor navigation.
    /// Applies only to the `WALK` travel mode.
    #[prost(bool, tag="4")]
    pub avoid_indoor: bool,
    /// Specifies the vehicle information.
    #[prost(message, optional, tag="5")]
    pub vehicle_info: ::core::option::Option<VehicleInfo>,
    /// Encapsulates information about toll passes.
    /// If toll passes are provided, the API tries to return the pass price. If
    /// toll passes are not provided, the API treats the toll pass as unknown and
    /// tries to return the cash price.
    /// Applies only to the DRIVE and TWO_WHEELER travel modes.
    #[prost(enumeration="TollPass", repeated, tag="6")]
    pub toll_passes: ::prost::alloc::vec::Vec<i32>,
}
/// Encapsulates the vehicle information, such as the license plate last
/// character.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleInfo {
    /// Specifies the license plate last character. Could be a digit or a letter.
    #[prost(string, tag="1")]
    pub license_plate_last_character: ::prost::alloc::string::String,
    /// Describes the vehicle's emission type.
    /// Applies only to the DRIVE travel mode.
    #[prost(enumeration="VehicleEmissionType", tag="2")]
    pub emission_type: i32,
}
/// A set of values used to specify the mode of travel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouteTravelMode {
    /// No travel mode specified. Defaults to `DRIVE`.
    TravelModeUnspecified = 0,
    /// Travel by passenger car.
    Drive = 1,
    /// Travel by bicycle.
    Bicycle = 2,
    /// Travel by walking.
    Walk = 3,
    /// Two-wheeled, motorized vehicle. For example, motorcycle. Note that this
    /// differs from the `BICYCLE` travel mode which covers human-powered mode.
    TwoWheeler = 4,
    /// Travel by licensed taxi, which may allow the vehicle to travel on
    /// designated taxi lanes in some areas.
    Taxi = 5,
}
/// A set of values that specify factors to take into consideration when
/// calculating the route.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoutingPreference {
    /// No routing preference specified. Default to `TRAFFIC_AWARE`.
    Unspecified = 0,
    /// Computes routes without taking traffic conditions into consideration.
    /// Suitable when traffic conditions don't matter. Using this value produces
    /// the lowest latency.
    TrafficUnaware = 1,
    /// Calculates routes taking traffic conditions into consideration. In contrast
    /// to `TRAFFIC_AWARE_OPTIMAL`, some optimizations are applied to significantly
    /// reduce latency.
    TrafficAware = 2,
    /// Calculates the routes taking traffic conditions into consideration,
    /// without applying most performance optimizations. Using this value produces
    /// the highest latency.
    TrafficAwareOptimal = 3,
}
/// A set of values that specify the unit of measure used in the display.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Units {
    /// Units of measure not specified. Defaults to the unit of measure inferred
    /// from the request.
    Unspecified = 0,
    /// Metric units of measure.
    Metric = 1,
    /// Imperial (English) units of measure.
    Imperial = 2,
}
/// ComputeCustomRoutes request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeCustomRoutesRequest {
    /// Required. Origin waypoint.
    #[prost(message, optional, tag="1")]
    pub origin: ::core::option::Option<Waypoint>,
    /// Required. Destination waypoint.
    #[prost(message, optional, tag="2")]
    pub destination: ::core::option::Option<Waypoint>,
    /// Optional. A set of waypoints along the route (excluding terminal points), for either
    /// stopping at or passing by. Up to 25 intermediate waypoints are supported.
    #[prost(message, repeated, tag="3")]
    pub intermediates: ::prost::alloc::vec::Vec<Waypoint>,
    /// Optional. Specifies the mode of transportation. Only DRIVE is supported now.
    #[prost(enumeration="RouteTravelMode", tag="4")]
    pub travel_mode: i32,
    /// Optional. Specifies how to compute the route. The server attempts to use the selected
    /// routing preference to compute the route. If the routing preference results
    /// in an error or an extra long latency, then an error is returned. In the
    /// future, we might implement a fallback mechanism to use a different option
    /// when the preferred option does not give a valid result. You can specify
    /// this option only when the `travel_mode` is `DRIVE` or `TWO_WHEELER`,
    /// otherwise the request fails.
    #[prost(enumeration="RoutingPreference", tag="5")]
    pub routing_preference: i32,
    /// Optional. Specifies your preference for the quality of the polyline.
    #[prost(enumeration="PolylineQuality", tag="6")]
    pub polyline_quality: i32,
    /// Optional. Specifies the preferred encoding for the polyline.
    #[prost(enumeration="PolylineEncoding", tag="13")]
    pub polyline_encoding: i32,
    /// Optional. The departure time. If you don't set this value, then this value
    /// defaults to the time that you made the request. If you set this value to a
    /// time that has already occurred, then the request fails.
    #[prost(message, optional, tag="7")]
    pub departure_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. A set of conditions to satisfy that affect the way routes are calculated.
    #[prost(message, optional, tag="11")]
    pub route_modifiers: ::core::option::Option<RouteModifiers>,
    /// Required. A route objective to optimize for.
    #[prost(message, optional, tag="12")]
    pub route_objective: ::core::option::Option<RouteObjective>,
    /// Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.> See
    /// [Language Support](<https://developers.google.com/maps/faq#languagesupport>)
    /// for the list of supported languages. When you don't provide this value, the
    /// display language is inferred from the location of the route request.
    #[prost(string, tag="9")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. Specifies the units of measure for the display fields. This includes the
    /// `instruction` field in `NavigationInstruction`. The units of measure used
    /// for the route, leg, step distance, and duration are not affected by this
    /// value. If you don't provide this value, then the display units are inferred
    /// from the location of the request.
    #[prost(enumeration="Units", tag="10")]
    pub units: i32,
}
/// Encapsulates an objective to optimize for by ComputeCustomRoutes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteObjective {
    /// The route objective.
    #[prost(oneof="route_objective::Objective", tags="1")]
    pub objective: ::core::option::Option<route_objective::Objective>,
}
/// Nested message and enum types in `RouteObjective`.
pub mod route_objective {
    /// Encapsulates a RateCard route objective.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RateCard {
        /// Optional. Cost per minute.
        #[prost(message, optional, tag="2")]
        pub cost_per_minute: ::core::option::Option<rate_card::MonetaryCost>,
        /// Optional. Cost per kilometer.
        #[prost(message, optional, tag="3")]
        pub cost_per_km: ::core::option::Option<rate_card::MonetaryCost>,
        /// Optional. Whether to include toll cost in the overall cost.
        #[prost(bool, tag="4")]
        pub include_tolls: bool,
    }
    /// Nested message and enum types in `RateCard`.
    pub mod rate_card {
        /// Encapsulates the cost used in the rate card.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MonetaryCost {
            /// Required. The cost value in local currency inferred from the request.
            #[prost(double, tag="1")]
            pub value: f64,
        }
    }
    /// The route objective.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Objective {
        /// The RateCard objective.
        #[prost(message, tag="1")]
        RateCard(RateCard),
    }
}
/// Encapsulates a route, which consists of a series of connected road segments
/// that join beginning, ending, and intermediate waypoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    /// A collection of legs (path segments between waypoints) that make-up the
    /// route. Each leg corresponds to the trip between two non-`via` Waypoints.
    /// For example, a route with no intermediate waypoints has only one leg. A
    /// route that includes one non-`via` intermediate waypoint has two legs. A
    /// route that includes one `via` intermediate waypoint has one leg. The order
    /// of the legs matches the order of Waypoints from `origin` to `intermediates`
    /// to `destination`.
    #[prost(message, repeated, tag="1")]
    pub legs: ::prost::alloc::vec::Vec<RouteLeg>,
    /// The travel distance of the route, in meters.
    #[prost(int32, tag="2")]
    pub distance_meters: i32,
    /// The length of time needed to navigate the route. If you set the
    /// `route_preference` to `TRAFFIC_UNAWARE`, then this value is the same as
    /// `static_duration`. If you set the `route_preference` to either
    /// `TRAFFIC_AWARE` or `TRAFFIC_AWARE_OPTIMAL`, then this value is calculated
    /// taking traffic conditions into account.
    #[prost(message, optional, tag="3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// The duration of traveling through the route without taking traffic
    /// conditions into consideration.
    #[prost(message, optional, tag="4")]
    pub static_duration: ::core::option::Option<::prost_types::Duration>,
    /// The overall route polyline. This polyline will be the combined polyline of
    /// all `legs`.
    #[prost(message, optional, tag="5")]
    pub polyline: ::core::option::Option<Polyline>,
    /// A description of the route.
    #[prost(string, tag="6")]
    pub description: ::prost::alloc::string::String,
    /// An array of warnings to show when displaying the route.
    #[prost(string, repeated, tag="7")]
    pub warnings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The viewport bounding box of the polyline.
    #[prost(message, optional, tag="8")]
    pub viewport: ::core::option::Option<super::super::super::geo::r#type::Viewport>,
    /// Additional information about the route.
    #[prost(message, optional, tag="9")]
    pub travel_advisory: ::core::option::Option<RouteTravelAdvisory>,
}
/// Encapsulates the additional information that the user should be informed
/// about, such as possible traffic zone restriction etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTravelAdvisory {
    /// The traffic restriction that applies to the route. A vehicle that is
    /// subject to the restriction is not allowed to travel on the route. As of
    /// October 2019, only Jakarta, Indonesia takes into consideration.
    #[prost(message, optional, tag="1")]
    pub traffic_restriction: ::core::option::Option<TrafficRestriction>,
    /// Encapsulates information about tolls on the Route.
    /// This field is only populated if we expect there are tolls on the Route.
    /// If this field is set but the estimated_price subfield is not populated,
    /// we expect that road contains tolls but we do not know an estimated price.
    /// If this field is not set, then we expect there is no toll on the Route.
    #[prost(message, optional, tag="2")]
    pub toll_info: ::core::option::Option<TollInfo>,
    /// Speed reading intervals detailing traffic density. Applicable in case of
    /// `TRAFFIC_AWARE` and `TRAFFIC_AWARE_OPTIMAL` routing preferences.
    /// The intervals cover the entire polyline of the route without overlap.
    /// The start point of a specified interval is the same as the end point of the
    /// preceding interval.
    ///
    /// Example:
    ///
    ///     polyline: A ---- B ---- C ---- D ---- E ---- F ---- G
    ///     speed_reading_intervals: [A,C), [C,D), [D,G).
    #[prost(message, repeated, tag="3")]
    pub speed_reading_intervals: ::prost::alloc::vec::Vec<SpeedReadingInterval>,
}
/// Encapsulates the additional information that the user should be informed
/// about, such as possible traffic zone restriction etc. on a route leg.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteLegTravelAdvisory {
    /// Encapsulates information about tolls on the specific RouteLeg.
    /// This field is only populated if we expect there are tolls on the RouteLeg.
    /// If this field is set but the estimated_price subfield is not populated,
    /// we expect that road contains tolls but we do not know an estimated price.
    /// If this field does not exist, then there is no toll on the RouteLeg.
    #[prost(message, optional, tag="1")]
    pub toll_info: ::core::option::Option<TollInfo>,
    /// Speed reading intervals detailing traffic density. Applicable in case of
    /// `TRAFFIC_AWARE` and `TRAFFIC_AWARE_OPTIMAL` routing preferences.
    /// The intervals cover the entire polyline of the RouteLg without overlap.
    /// The start point of a specified interval is the same as the end point of the
    /// preceding interval.
    ///
    /// Example:
    ///
    ///     polyline: A ---- B ---- C ---- D ---- E ---- F ---- G
    ///     speed_reading_intervals: [A,C), [C,D), [D,G).
    #[prost(message, repeated, tag="2")]
    pub speed_reading_intervals: ::prost::alloc::vec::Vec<SpeedReadingInterval>,
}
/// Encapsulates the additional information that the user should be informed
/// about, such as possible traffic zone restriction on a leg step.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteLegStepTravelAdvisory {
    /// Speed reading intervals detailing traffic density. Applicable in case of
    /// `TRAFFIC_AWARE` and `TRAFFIC_AWARE_OPTIMAL` routing preferences.
    /// The intervals cover the entire polyline of the RouteLegStep without
    /// overlap. The start point of a specified interval is the same as the end
    /// point of the preceding interval.
    ///
    /// Example:
    ///
    ///     polyline: A ---- B ---- C ---- D ---- E ---- F ---- G
    ///     speed_reading_intervals: [A,C), [C,D), [D,G).
    #[prost(message, repeated, tag="1")]
    pub speed_reading_intervals: ::prost::alloc::vec::Vec<SpeedReadingInterval>,
}
/// Encapsulates the traffic restriction applied to the route. As of October
/// 2019, only Jakarta, Indonesia takes into consideration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficRestriction {
    /// The restriction based on the vehicle's license plate last character. If
    /// this field does not exist, then no restriction on route.
    #[prost(message, optional, tag="1")]
    pub license_plate_last_character_restriction: ::core::option::Option<LicensePlateLastCharacterRestriction>,
}
/// Encapsulates the license plate last character restriction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LicensePlateLastCharacterRestriction {
    /// The allowed last character of a license plate of a vehicle. Only vehicles
    /// whose license plate's last characters match these are allowed to travel on
    /// the route. If empty, no vehicle is allowed.
    #[prost(string, repeated, tag="1")]
    pub allowed_last_characters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Encapsulates a segment between non-`via` waypoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteLeg {
    /// The travel distance of the route leg, in meters.
    #[prost(int32, tag="1")]
    pub distance_meters: i32,
    /// The length of time needed to navigate the leg. If the `route_preference`
    /// is set to `TRAFFIC_UNAWARE`, then this value is the same as
    /// `static_duration`. If the `route_preference` is either `TRAFFIC_AWARE` or
    /// `TRAFFIC_AWARE_OPTIMAL`, then this value is calculated taking traffic
    /// conditions into account.
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// The duration of traveling through the leg, calculated without taking
    /// traffic conditions into consideration.
    #[prost(message, optional, tag="3")]
    pub static_duration: ::core::option::Option<::prost_types::Duration>,
    /// The overall polyline for this leg. This includes that each `step`'s
    /// polyline.
    #[prost(message, optional, tag="4")]
    pub polyline: ::core::option::Option<Polyline>,
    /// The start location of this leg. This might be different from the provided
    /// `origin`. For example, when the provided `origin` is not near a road, this
    /// is a point on the road.
    #[prost(message, optional, tag="5")]
    pub start_location: ::core::option::Option<Location>,
    /// The end location of this leg. This might be different from the provided
    /// `destination`. For example, when the provided `destination` is not near a
    /// road, this is a point on the road.
    #[prost(message, optional, tag="6")]
    pub end_location: ::core::option::Option<Location>,
    /// An array of steps denoting segments within this leg. Each step represents
    /// one navigation instruction.
    #[prost(message, repeated, tag="7")]
    pub steps: ::prost::alloc::vec::Vec<RouteLegStep>,
    /// Encapsulates the additional information that the user should be informed
    /// about, such as possible traffic zone restriction etc. on a route leg.
    #[prost(message, optional, tag="8")]
    pub travel_advisory: ::core::option::Option<RouteLegTravelAdvisory>,
}
/// Encapsulates toll information on a `Route` or on a `RouteLeg`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TollInfo {
    /// The monetary amount of tolls for the corresponding Route or RouteLeg.
    /// This list contains a money amount for each currency that is expected
    /// to be charged by the toll stations. Typically this list will contain only
    /// one item for routes with tolls in one currency. For international trips,
    /// this list may contain multiple items to reflect tolls in different
    /// currencies.
    #[prost(message, repeated, tag="1")]
    pub estimated_price: ::prost::alloc::vec::Vec<super::super::super::r#type::Money>,
}
/// Encapsulates a segment of a `RouteLeg`. A step corresponds to a single
/// navigation instruction. Route legs are made up of steps.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteLegStep {
    /// The travel distance of this step, in meters. In some circumstances, this
    /// field might not have a value.
    #[prost(int32, tag="1")]
    pub distance_meters: i32,
    /// The duration of travel through this step without taking traffic conditions
    /// into consideration. In some circumstances, this field might not have a
    /// value.
    #[prost(message, optional, tag="2")]
    pub static_duration: ::core::option::Option<::prost_types::Duration>,
    /// The polyline associated with this step.
    #[prost(message, optional, tag="3")]
    pub polyline: ::core::option::Option<Polyline>,
    /// The start location of this step.
    #[prost(message, optional, tag="4")]
    pub start_location: ::core::option::Option<Location>,
    /// The end location of this step.
    #[prost(message, optional, tag="5")]
    pub end_location: ::core::option::Option<Location>,
    /// Navigation instructions.
    #[prost(message, optional, tag="6")]
    pub navigation_instruction: ::core::option::Option<NavigationInstruction>,
    /// Encapsulates the additional information that the user should be informed
    /// about, such as possible traffic zone restriction on a leg step.
    #[prost(message, optional, tag="7")]
    pub travel_advisory: ::core::option::Option<RouteLegStepTravelAdvisory>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NavigationInstruction {
    /// Encapsulates the navigation instructions for the current step (e.g., turn
    /// left, merge, straight, etc.). This field determines which icon to display.
    #[prost(enumeration="Maneuver", tag="1")]
    pub maneuver: i32,
    /// Instructions for navigating this step.
    #[prost(string, tag="2")]
    pub instructions: ::prost::alloc::string::String,
}
/// Traffic density indicator on a contiguous segment of a polyline or path.
/// Given a path with points P_0, P_1, ... , P_N (zero-based index), the
/// SpeedReadingInterval defines an interval and describes its traffic using the
/// following categories.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeedReadingInterval {
    /// The starting index of this interval in the polyline.
    /// In JSON, when the index is 0, the field appears to be unpopulated.
    #[prost(int32, tag="1")]
    pub start_polyline_point_index: i32,
    /// The ending index of this interval in the polyline.
    /// In JSON, when the index is 0, the field appears to be unpopulated.
    #[prost(int32, tag="2")]
    pub end_polyline_point_index: i32,
    /// Traffic speed in this interval.
    #[prost(enumeration="speed_reading_interval::Speed", tag="3")]
    pub speed: i32,
}
/// Nested message and enum types in `SpeedReadingInterval`.
pub mod speed_reading_interval {
    /// The classification of polyline speed based on traffic data.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Speed {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Normal speed, no slowdown is detected.
        Normal = 1,
        /// Slowdown detected, but no traffic jam formed.
        Slow = 2,
        /// Traffic jam detected.
        TrafficJam = 3,
    }
}
/// A set of values that specify the navigation action to take for the current
/// step (e.g., turn left, merge, straight, etc.).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Maneuver {
    /// Not used.
    Unspecified = 0,
    /// Turn slightly to the left.
    TurnSlightLeft = 1,
    /// Turn sharply to the left.
    TurnSharpLeft = 2,
    /// Make a left u-turn.
    UturnLeft = 3,
    /// Turn left.
    TurnLeft = 4,
    /// Turn slightly to the right.
    TurnSlightRight = 5,
    /// Turn sharply to the right.
    TurnSharpRight = 6,
    /// Make a right u-turn.
    UturnRight = 7,
    /// Turn right.
    TurnRight = 8,
    /// Go straight.
    Straight = 9,
    /// Take the left ramp.
    RampLeft = 10,
    /// Take the right ramp.
    RampRight = 11,
    /// Merge into traffic.
    Merge = 12,
    /// Take the left fork.
    ForkLeft = 13,
    /// Take the right fork.
    ForkRight = 14,
    /// Take the ferry.
    Ferry = 15,
    /// Take the train leading onto the ferry.
    FerryTrain = 16,
    /// Turn left at the roundabout.
    RoundaboutLeft = 17,
    /// Turn right at the roundabout.
    RoundaboutRight = 18,
}
/// Encapsulates a custom route computed based on the route objective specified
/// by the customer. CustomRoute contains a route and a route token, which can be
/// passed to NavSDK to reconstruct the custom route for turn by turn navigation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomRoute {
    /// The route considered 'best' for the input route objective.
    #[prost(message, optional, tag="11")]
    pub route: ::core::option::Option<Route>,
    /// Web-safe base64 encoded route token that can be passed to NavSDK, which
    /// allows NavSDK to reconstruct the route during navigation, and in the event
    /// of rerouting honor the original intention when RoutesPreferred
    /// ComputeCustomRoutes is called. Customers should treat this token as an
    /// opaque blob.
    #[prost(string, tag="12")]
    pub token: ::prost::alloc::string::String,
}
/// Information related to how and why a fallback result was used. If this field
/// is set, then it means the server used a different routing mode from your
/// preferred mode as fallback.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FallbackInfo {
    /// Routing mode used for the response. If fallback was triggered, the mode
    /// may be different from routing preference set in the original client
    /// request.
    #[prost(enumeration="FallbackRoutingMode", tag="1")]
    pub routing_mode: i32,
    /// The reason why fallback response was used instead of the original response.
    /// This field is only populated when the fallback mode is triggered and the
    /// fallback response is returned.
    #[prost(enumeration="FallbackReason", tag="2")]
    pub reason: i32,
}
/// Reasons for using fallback response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FallbackReason {
    /// No fallback reason specified.
    Unspecified = 0,
    /// A server error happened while calculating routes with your preferred
    /// routing mode, but we were able to return a result calculated by an
    /// alternative mode.
    ServerError = 1,
    /// We were not able to finish the calculation with your preferred routing mode
    /// on time, but we were able to return a result calculated by an alternative
    /// mode.
    LatencyExceeded = 2,
}
/// Actual routing mode used for returned fallback response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FallbackRoutingMode {
    /// Not used.
    Unspecified = 0,
    /// Indicates the "TRAFFIC_UNAWARE" routing mode was used to compute the
    /// response.
    FallbackTrafficUnaware = 1,
    /// Indicates the "TRAFFIC_AWARE" routing mode was used to compute the
    /// response.
    FallbackTrafficAware = 2,
}
/// ComputeCustomRoutes response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeCustomRoutesResponse {
    /// The ‘best’ routes for the input route objective.
    #[prost(message, repeated, tag="7")]
    pub routes: ::prost::alloc::vec::Vec<CustomRoute>,
    /// The fastest reference route.
    #[prost(message, optional, tag="5")]
    pub fastest_route: ::core::option::Option<CustomRoute>,
    /// The shortest reference route.
    #[prost(message, optional, tag="6")]
    pub shortest_route: ::core::option::Option<CustomRoute>,
    /// Fallback info for custom routes.
    #[prost(message, optional, tag="8")]
    pub fallback_info: ::core::option::Option<compute_custom_routes_response::FallbackInfo>,
}
/// Nested message and enum types in `ComputeCustomRoutesResponse`.
pub mod compute_custom_routes_response {
    /// Encapsulates fallback info for ComputeCustomRoutes. ComputeCustomRoutes
    /// performs two types of fallbacks:
    ///
    /// 1. If it cannot compute the route using the routing_preference requested by
    /// the customer, it will fallback to another routing mode. In this case
    /// fallback_routing_mode and routing_mode_fallback_reason are used to
    /// communicate the fallback routing mode used, as well as the reason for
    /// fallback.
    ///
    /// 2. If it cannot compute a 'best' route for the route objective specified by
    /// the customer, it might fallback to another objective.
    /// fallback_route_objective is used to communicate the fallback route
    /// objective.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FallbackInfo {
        /// Routing mode used for the response. If fallback was triggered, the mode
        /// may be different from routing preference set in the original client
        /// request.
        #[prost(enumeration="super::FallbackRoutingMode", tag="1")]
        pub routing_mode: i32,
        /// The reason why fallback response was used instead of the original
        /// response.
        /// This field is only populated when the fallback mode is triggered and
        /// the fallback response is returned.
        #[prost(enumeration="super::FallbackReason", tag="2")]
        pub routing_mode_reason: i32,
        /// The route objective used for the response. If fallback was triggered, the
        /// objective may be different from the route objective provided in the
        /// original client request.
        #[prost(enumeration="fallback_info::FallbackRouteObjective", tag="3")]
        pub route_objective: i32,
    }
    /// Nested message and enum types in `FallbackInfo`.
    pub mod fallback_info {
        /// RouteObjective used for the response.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum FallbackRouteObjective {
            /// Fallback route objective unspecified.
            Unspecified = 0,
            /// If customer requests RateCard and sets include_tolls to true, and
            /// Google does not have toll price data for the route, the API falls back
            /// to RateCard without considering toll price.
            FallbackRatecardWithoutTollPriceData = 1,
        }
    }
}
/// ComputeRouteMatrix request message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeRouteMatrixRequest {
    /// Required. Array of origins, which determines the rows of the response matrix.
    /// Several size restrictions apply to the cardinality of origins and
    /// destinations:
    ///
    /// * The number of elements (origins × destinations) must be no greater than
    /// 625 in any case.
    /// * The number of elements (origins × destinations) must be no greater than
    /// 100 if routing_preference is set to `TRAFFIC_AWARE_OPTIMAL`.
    /// * The number of waypoints (origins + destinations) specified as `place_id`
    /// must be no greater than 50.
    #[prost(message, repeated, tag="1")]
    pub origins: ::prost::alloc::vec::Vec<RouteMatrixOrigin>,
    /// Required. Array of destinations, which determines the columns of the response matrix.
    #[prost(message, repeated, tag="2")]
    pub destinations: ::prost::alloc::vec::Vec<RouteMatrixDestination>,
    /// Optional. Specifies the mode of transportation.
    #[prost(enumeration="RouteTravelMode", tag="3")]
    pub travel_mode: i32,
    /// Optional. Specifies how to compute the route. The server attempts to use the selected
    /// routing preference to compute the route. If the routing preference results
    /// in an error or an extra long latency, an error is returned. In the future,
    /// we might implement a fallback mechanism to use a different option when the
    /// preferred option does not give a valid result. You can specify this option
    /// only when the `travel_mode` is `DRIVE` or `TWO_WHEELER`, otherwise the
    /// request fails.
    #[prost(enumeration="RoutingPreference", tag="4")]
    pub routing_preference: i32,
    /// Optional. The departure time. If you don't set this value, this defaults to the time
    /// that you made the request. If you set this value to a time that has already
    /// occurred, the request fails.
    #[prost(message, optional, tag="5")]
    pub departure_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A single origin for ComputeRouteMatrixRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMatrixOrigin {
    /// Required. Origin waypoint
    #[prost(message, optional, tag="1")]
    pub waypoint: ::core::option::Option<Waypoint>,
    /// Optional. Modifiers for every route that takes this as the origin
    #[prost(message, optional, tag="2")]
    pub route_modifiers: ::core::option::Option<RouteModifiers>,
}
/// A single destination for ComputeRouteMatrixRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMatrixDestination {
    /// Required. Destination waypoint
    #[prost(message, optional, tag="1")]
    pub waypoint: ::core::option::Option<Waypoint>,
}
/// ComputeRoutes the response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeRoutesResponse {
    /// Contains an array of computed routes (up to three) when you specify
    /// compute_alternatives_routes, and contains just one route when you don't.
    /// When this array contains multiple entries, the first one is the most
    /// recommended route. If the array is empty, then it means no route could be
    /// found.
    #[prost(message, repeated, tag="1")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
    /// In some cases when the server is not able to compute the route results with
    /// all of the input preferences, it may fallback to using a different way of
    /// computation. When fallback mode is used, this field contains detailed info
    /// about the fallback response. Otherwise this field is unset.
    #[prost(message, optional, tag="2")]
    pub fallback_info: ::core::option::Option<FallbackInfo>,
}
/// Encapsulates route information computed for an origin/destination pair in the
/// ComputeRouteMatrix API. This proto can be streamed to the client.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMatrixElement {
    /// Zero-based index of the origin in the request.
    #[prost(int32, tag="1")]
    pub origin_index: i32,
    /// Zero-based index of the destination in the request.
    #[prost(int32, tag="2")]
    pub destination_index: i32,
    /// Error status code for this element.
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// Indicates whether the route was found or not. Independent of status.
    #[prost(enumeration="RouteMatrixElementCondition", tag="9")]
    pub condition: i32,
    /// The travel distance of the route, in meters.
    #[prost(int32, tag="4")]
    pub distance_meters: i32,
    /// The length of time needed to navigate the route. If you set the
    /// `route_preference` to `TRAFFIC_UNAWARE`, then this value is the same as
    /// `static_duration`. If you set the `route_preference` to either
    /// `TRAFFIC_AWARE` or `TRAFFIC_AWARE_OPTIMAL`, then this value is calculated
    /// taking traffic conditions into account.
    #[prost(message, optional, tag="5")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// The duration of traveling through the route without taking traffic
    /// conditions into consideration.
    #[prost(message, optional, tag="6")]
    pub static_duration: ::core::option::Option<::prost_types::Duration>,
    /// Additional information about the route. For example: restriction
    /// information and toll information
    #[prost(message, optional, tag="7")]
    pub travel_advisory: ::core::option::Option<RouteTravelAdvisory>,
    /// In some cases when the server is not able to compute the route with the
    /// given preferences for this particular origin/destination pair, it may
    /// fall back to using a different mode of computation. When fallback mode is
    /// used, this field contains detailed information about the fallback response.
    /// Otherwise this field is unset.
    #[prost(message, optional, tag="8")]
    pub fallback_info: ::core::option::Option<FallbackInfo>,
}
/// The condition of the route being returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouteMatrixElementCondition {
    /// Only used when the `status` of the element is not OK.
    Unspecified = 0,
    /// A route was found, and the corresponding information was filled out for the
    /// element.
    RouteExists = 1,
    /// No route could be found. Fields containing route information, such as
    /// `distance_meters` or `duration`, will not be filled out in the element.
    RouteNotFound = 2,
}
/// Generated client implementations.
pub mod routes_preferred_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// The Routes Preferred API.
    #[derive(Debug, Clone)]
    pub struct RoutesPreferredClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RoutesPreferredClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RoutesPreferredClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            RoutesPreferredClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Returns the primary route along with optional alternate routes, given a set
        /// of terminal and intermediate waypoints.
        ///
        /// **NOTE:** This method requires that you specify a response field mask in
        /// the input. You can provide the response field mask by using URL parameter
        /// `$fields` or `fields`, or by using an HTTP/gRPC header `X-Goog-FieldMask`
        /// (see the [available URL parameters and
        /// headers](https://cloud.google.com/apis/docs/system-parameters). The value
        /// is a comma separated list of field paths. See detailed documentation about
        /// [how to construct the field
        /// paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto).
        ///
        /// For example, in this method:
        ///
        /// * Field mask of all available fields (for manual inspection):
        ///   `X-Goog-FieldMask: *`
        /// * Field mask of Route-level duration, distance, and polyline (an example
        /// production setup):
        ///   `X-Goog-FieldMask:
        ///   routes.duration,routes.distanceMeters,routes.polyline.encodedPolyline`
        ///
        /// Google discourage the use of the wildcard (`*`) response field mask, or
        /// specifying the field mask at the top level (`routes`), because:
        ///
        /// * Selecting only the fields that you need helps our server save computation
        /// cycles, allowing us to return the result to you with a lower latency.
        /// * Selecting only the fields that you need
        /// in your production job ensures stable latency performance. We might add
        /// more response fields in the future, and those new fields might require
        /// extra computation time. If you select all fields, or if you select all
        /// fields at the top level, then you might experience performance degradation
        /// because any new field we add will be automatically included in the
        /// response.
        /// * Selecting only the fields that you need results in a smaller response
        /// size, and thus higher network throughput.
        pub async fn compute_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeRoutesRequest>,
        ) -> Result<tonic::Response<super::ComputeRoutesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.routes.v1.RoutesPreferred/ComputeRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Takes in a list of origins and destinations and returns a stream containing
        /// route information for each combination of origin and destination.
        ///
        /// **NOTE:** This method requires that you specify a response field mask in
        /// the input. You can provide the response field mask by using the URL
        /// parameter `$fields` or `fields`, or by using the HTTP/gRPC header
        /// `X-Goog-FieldMask` (see the [available URL parameters and
        /// headers](https://cloud.google.com/apis/docs/system-parameters). The value
        /// is a comma separated list of field paths. See this detailed documentation
        /// about [how to construct the field
        /// paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto).
        ///
        /// For example, in this method:
        ///
        /// * Field mask of all available fields (for manual inspection):
        ///   `X-Goog-FieldMask: *`
        /// * Field mask of route durations, distances, element status, condition, and
        ///   element indices (an example production setup):
        ///   `X-Goog-FieldMask:
        ///   originIndex,destinationIndex,status,condition,distanceMeters,duration`
        ///
        /// It is critical that you include `status` in your field mask as otherwise
        /// all messages will appear to be OK. Google discourages the use of the
        /// wildcard (`*`) response field mask, because:
        ///
        /// * Selecting only the fields that you need helps our server save computation
        /// cycles, allowing us to return the result to you with a lower latency.
        /// * Selecting only the fields that you need in your production job ensures
        /// stable latency performance. We might add more response fields in the
        /// future, and those new fields might require extra computation time. If you
        /// select all fields, or if you select all fields at the top level, then you
        /// might experience performance degradation because any new field we add will
        /// be automatically included in the response.
        /// * Selecting only the fields that you need results in a smaller response
        /// size, and thus higher network throughput.
        pub async fn compute_route_matrix(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeRouteMatrixRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::RouteMatrixElement>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.routes.v1.RoutesPreferred/ComputeRouteMatrix",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Given a set of terminal and intermediate waypoints, and a route objective,
        /// computes the best route for the route objective. Also returns fastest route
        /// and shortest route as reference routes.
        ///
        /// **NOTE:** This method requires that you specify a response field mask in
        /// the input. You can provide the response field mask by using the URL
        /// parameter `$fields` or `fields`, or by using the HTTP/gRPC header
        /// `X-Goog-FieldMask` (see the [available URL parameters and
        /// headers](https://cloud.google.com/apis/docs/system-parameters). The value
        /// is a comma separated list of field paths. See this detailed documentation
        /// about [how to construct the field
        /// paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto).
        ///
        /// For example, in this method:
        ///
        /// * Field mask of all available fields (for manual inspection):
        ///   `X-Goog-FieldMask: *`
        /// * Field mask of route distances, durations, token and toll info:
        ///   `X-Goog-FieldMask:
        ///   routes.route.distanceMeters,routes.route.duration,routes.token,routes.route.travelAdvisory.tollInfo`
        ///
        /// Google discourages the use of the wildcard (`*`) response field mask, or
        /// specifying the field mask at the top level (`routes`), because:
        ///
        /// * Selecting only the fields that you need helps our server save computation
        /// cycles, allowing us to return the result to you with a lower latency.
        /// * Selecting only the fields that you need in your production job ensures
        /// stable latency performance. We might add more response fields in the
        /// future, and those new fields might require extra computation time. If you
        /// select all fields, or if you select all fields at the top level, then you
        /// might experience performance degradation because any new field we add will
        /// be automatically included in the response.
        /// * Selecting only the fields that you need results in a smaller response
        /// size, and thus higher network throughput.
        pub async fn compute_custom_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeCustomRoutesRequest>,
        ) -> Result<tonic::Response<super::ComputeCustomRoutesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.routes.v1.RoutesPreferred/ComputeCustomRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
