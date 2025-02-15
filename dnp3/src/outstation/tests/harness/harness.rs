use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

use crate::decode::AppDecodeLevel;
use crate::link::header::{BroadcastConfirmMode, FrameInfo, FrameType};
use crate::link::{EndpointAddress, LinkErrorMode};
use crate::outstation::config::{Feature, OutstationConfig};
use crate::outstation::database::EventBufferConfig;
use crate::outstation::session::RunError;
use crate::outstation::task::OutstationTask;
use crate::outstation::tests::harness::{
    event_handlers, ApplicationData, Event, EventReceiver, MockControlHandler,
    MockOutstationApplication, MockOutstationInformation,
};
use crate::outstation::OutstationHandle;
use crate::util::phys::PhysLayer;

pub(crate) fn get_default_config() -> OutstationConfig {
    let mut config = get_default_unsolicited_config();
    config.features.unsolicited = Feature::Disabled;
    config
}

pub(crate) fn get_default_unsolicited_config() -> OutstationConfig {
    let mut config = OutstationConfig::new(
        EndpointAddress::try_new(10).unwrap(),
        EndpointAddress::try_new(1).unwrap(),
        EventBufferConfig::all_types(5),
    );

    config.decode_level = AppDecodeLevel::ObjectValues.into();

    config
}

pub(crate) struct OutstationHarness {
    pub(crate) handle: OutstationHandle,
    pub(crate) io: tokio_mock_io::Handle,
    task: JoinHandle<RunError>,
    events: EventReceiver,
    pub(crate) application_data: Arc<Mutex<ApplicationData>>,
}

impl OutstationHarness {
    pub(crate) async fn test_request_response(&mut self, request: &[u8], response: &[u8]) {
        self.send_and_process(request).await;
        self.expect_response(response).await;
    }

    pub(crate) async fn expect_response(&mut self, response: &[u8]) {
        assert_eq!(
            self.io.next_event().await,
            tokio_mock_io::Event::Write(response.to_vec())
        );
    }

    pub(crate) async fn send_and_process(&mut self, request: &[u8]) {
        self.io.read(request);
        assert_eq!(self.io.next_event().await, tokio_mock_io::Event::Read);
    }

    pub(crate) async fn wait_for_events(&mut self, expected: &[Event]) {
        for event in expected {
            let next = self.events.next().await;
            if next != *event {
                panic!("Expected {:?} but next event is {:?}", event, next);
            }
        }
    }

    pub(crate) fn check_events(&mut self, expected: &[Event]) {
        for event in expected {
            match self.events.poll() {
                Some(next) => {
                    if next != *event {
                        panic!("Expected {:?} but next event is {:?}", event, next)
                    }
                }
                None => panic!("Expected {:?} but no event ready", event),
            }
        }
    }

    pub(crate) fn check_no_events(&mut self) {
        if let Some(x) = self.events.poll() {
            panic!("expected no events, but next event is: {:?}", x)
        }
    }
}

pub(crate) fn new_harness(config: OutstationConfig) -> OutstationHarness {
    new_harness_impl(config, None)
}

pub(crate) fn new_harness_with_custom_event_buffers(config: OutstationConfig) -> OutstationHarness {
    new_harness_impl(config, None)
}

pub(crate) fn new_harness_for_broadcast(
    config: OutstationConfig,
    broadcast: BroadcastConfirmMode,
) -> OutstationHarness {
    new_harness_impl(config, Some(broadcast))
}

fn new_harness_impl(
    config: OutstationConfig,
    broadcast: Option<BroadcastConfirmMode>,
) -> OutstationHarness {
    let (sender, receiver) = event_handlers();

    let (data, application) = MockOutstationApplication::new(sender.clone());

    let (task, handle) = OutstationTask::create(
        LinkErrorMode::Close,
        config,
        application,
        MockOutstationInformation::new(sender.clone()),
        MockControlHandler::new(sender.clone()),
    );

    let mut task = Box::new(task);

    task.get_reader()
        .get_inner()
        .set_rx_frame_info(FrameInfo::new(
            config.master_address,
            broadcast,
            FrameType::Data,
        ));

    let (io, io_handle) = tokio_mock_io::mock();

    let mut io = PhysLayer::Mock(io);

    OutstationHarness {
        handle,
        io: io_handle,
        task: tokio::spawn(async move { task.run(&mut io).await }),
        events: receiver,
        application_data: data,
    }
}
