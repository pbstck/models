#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidRequest {
    #[prost(string, tag="1")]
    pub bidder_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub state: ::prost::alloc::string::String,
    #[prost(float, tag="3")]
    pub cpm: f32,
    #[prost(string, tag="4")]
    pub size: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub elapsed: u32,
    /// for join with impression
    #[prost(string, tag="6")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="7")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="8")]
    pub currency: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Auction {
    #[prost(uint64, tag="1")]
    pub ingestion_time: u64,
    #[prost(string, tag="2")]
    pub scope_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub tag_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub ad_unit: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub device: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub country: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub domain: ::prost::alloc::string::String,
    /// deduplication an join 
    #[prost(string, tag="8")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub auction_id: ::prost::alloc::string::String,
    /// bid objects
    #[prost(message, repeated, tag="10")]
    pub bid_requests: ::prost::alloc::vec::Vec<BidRequest>,
    /// indicate the iteration of the refresh
    #[prost(bool, tag="11")]
    pub refresh: bool,
    /// ex [ "refresh:3" ]
    #[prost(string, repeated, tag="12")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// debug fields
    #[prost(string, tag="13")]
    pub browser_name: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub browser_version: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub os_name: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub os_version: ::prost::alloc::string::String,
    /// extended model fields
    #[prost(string, tag="17")]
    pub href: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="18")]
    pub sizes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="19")]
    pub user_consent_state: ::prost::alloc::string::String,
    #[prost(string, tag="20")]
    pub user_consent_version: ::prost::alloc::string::String,
    #[prost(string, tag="21")]
    pub pbjs_version: ::prost::alloc::string::String,
    #[prost(string, tag="22")]
    pub pbstck_version: ::prost::alloc::string::String,
    #[prost(string, tag="23")]
    pub ad_unit_path: ::prost::alloc::string::String,
    #[prost(string, tag="24")]
    pub has_user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="25")]
    pub user_id_provider_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="26")]
    pub pubstack_managed: bool,
    #[prost(bool, tag="27")]
    pub pubstack_refresh: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Impression {
    #[prost(uint64, tag="1")]
    pub ingestion_time: u64,
    #[prost(string, tag="2")]
    pub scope_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub tag_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub ad_unit: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub device: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub country: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub domain: ::prost::alloc::string::String,
    /// deduplication an join 
    #[prost(string, tag="8")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub auction_id: ::prost::alloc::string::String,
    /// auction results
    #[prost(float, tag="10")]
    pub cpm: f32,
    #[prost(string, tag="11")]
    pub size: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub bidder_code: ::prost::alloc::string::String,
    /// indicate the iteration of the refresh
    #[prost(bool, tag="13")]
    pub refresh: bool,
    /// ex [ "refresh:3" ]
    #[prost(string, repeated, tag="14")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// for join with bids and deduplication
    #[prost(string, tag="15")]
    pub bid_id: ::prost::alloc::string::String,
    /// debug fields
    #[prost(string, tag="16")]
    pub browser_name: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub browser_version: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub os_name: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub os_version: ::prost::alloc::string::String,
    /// extended model fields
    #[prost(string, tag="20")]
    pub href: ::prost::alloc::string::String,
    #[prost(string, tag="21")]
    pub user_consent_state: ::prost::alloc::string::String,
    #[prost(string, tag="22")]
    pub user_consent_version: ::prost::alloc::string::String,
    #[prost(string, tag="23")]
    pub pbjs_version: ::prost::alloc::string::String,
    #[prost(string, tag="24")]
    pub pbstck_version: ::prost::alloc::string::String,
    #[prost(string, tag="25")]
    pub ad_unit_path: ::prost::alloc::string::String,
    #[prost(string, tag="26")]
    pub has_user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="27")]
    pub user_id_provider_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="28")]
    pub viewability_measurable: bool,
    #[prost(bool, tag="29")]
    pub pubstack_managed: bool,
    #[prost(bool, tag="30")]
    pub pubstack_refresh: bool,
    #[prost(string, tag="31")]
    pub currency: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewableImpression {
    /// Context
    #[prost(uint64, tag="1")]
    pub ingestion_time: u64,
    #[prost(string, tag="2")]
    pub scope_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub tag_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub country: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub device: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub domain: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub browser_name: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub browser_version: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub os_name: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub os_version: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub pbstck_version: ::prost::alloc::string::String,
    /// Impression inherited fields
    #[prost(string, tag="12")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub auction_id: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub ad_unit: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub ad_unit_path: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub bidder_code: ::prost::alloc::string::String,
    #[prost(float, tag="17")]
    pub cpm: f32,
    #[prost(bool, tag="18")]
    pub refresh: bool,
    #[prost(string, tag="19")]
    pub size: ::prost::alloc::string::String,
    #[prost(string, tag="20")]
    pub pbjs_version: ::prost::alloc::string::String,
    /// Viewability specific fields
    #[prost(string, tag="21")]
    pub html_element_id: ::prost::alloc::string::String,
    #[prost(bool, tag="22")]
    pub mrc_viewable: bool,
    #[prost(string, tag="23")]
    pub currency: ::prost::alloc::string::String,
}
