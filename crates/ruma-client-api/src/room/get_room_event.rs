//! `GET /_matrix/client/*/rooms/{roomId}/event/{eventId}`
//!
//! Get a single event based on roomId/eventId

pub mod v3 {
    //! `/v3/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/latest/client-server-api/#get_matrixclientv3roomsroomideventeventid

    use ruma_common::{
        api::{request, response, Metadata},
        metadata,
        serde::Raw,
        OwnedEventId, OwnedRoomId,
    };
    use ruma_events::AnyTimelineEvent;

    const METADATA: Metadata = metadata! {
        method: GET,
        rate_limited: false,
        authentication: AccessToken,
        history: {
            1.0 => "/_matrix/client/r0/rooms/{room_id}/event/{event_id}",
            1.1 => "/_matrix/client/v3/rooms/{room_id}/event/{event_id}",
        }
    };

    /// Request type for the `get_room_event` endpoint.
    #[request(error = crate::Error)]
    pub struct Request {
        /// The ID of the room the event is in.
        #[ruma_api(path)]
        pub room_id: OwnedRoomId,

        /// The ID of the event.
        #[ruma_api(path)]
        pub event_id: OwnedEventId,

        #[cfg(feature = "unstable-msc2815")]
        #[ruma_api(query)]
        #[serde(default, rename = "fi.mau.msc2815.include_unredacted_content")]
        /// Whether respond with the original event, even if it has been redacted.
        pub include_unredacted_content: bool,
    }

    /// Response type for the `get_room_event` endpoint.
    #[response(error = crate::Error)]
    pub struct Response {
        /// Arbitrary JSON of the event body.
        #[ruma_api(body)]
        pub event: Raw<AnyTimelineEvent>,
    }

    impl Request {
        /// Creates a new `Request` with the given room ID and event ID.
        pub fn new(room_id: OwnedRoomId, event_id: OwnedEventId) -> Self {
            Self {
                room_id,
                event_id,
                #[cfg(feature = "unstable-msc2815")]
                include_unredacted_content: false,
            }
        }
    }

    impl Response {
        /// Creates a new `Response` with the given event.
        pub fn new(event: Raw<AnyTimelineEvent>) -> Self {
            Self { event }
        }
    }
}
