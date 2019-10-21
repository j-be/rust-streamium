use crate::{StreamiumDbConn, NodeList};
use streamium_db::models::{Node, Nodetypes};
use streamium_db::repo;
use rocket::Data;
use rocket::response::Responder;
use rocket::{Response, Request};
use rocket::http::{ContentType, Status};
use std::io::{Cursor, Read};
use quick_xml::events::Event;
use simple_xml_serialize::XMLElement;
use quick_xml::Reader;

#[post("/", data = "<request_data>")]
pub fn get_nodes(conn: StreamiumDbConn, request_data: Data) -> NodeList {
    let mut data_str : String = String::new();
    request_data.open().read_to_string(&mut data_str)
        .expect("Failed to read request data");
    let nav_data = parse_request_data(data_str);
    println!("{:?}", nav_data);
    return NodeList{
        nodes: repo::get_nodes(&*conn, nav_data.nodeid, nav_data.fromindexelem, nav_data.numelem),
        totnumelem: repo::get_node_count(&*conn, nav_data.nodeid),
        fromindex: 0,
    };
}

#[derive(Debug)]
struct RequestNavData {
    service_id: i32,
    numelem: i64,
    nodeid: i32,
    fromindexelem: i64,
}

impl<'r> Responder<'r> for NodeList {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        let mut root = nodes_to_xml(&self.nodes);

        root.add_element(forge_xml_element("totnumelem", self.totnumelem.to_string()));
        root.add_element(forge_xml_element("fromindex", self.fromindex.to_string()));
        root.add_element(forge_xml_element("numelem", self.nodes.len().to_string()));

        Response::build()
            .header(ContentType::XML)
            .sized_body(Cursor::new(root.to_string()))
            .ok()
    }
}

fn parse_request_data(request_data: String) -> RequestNavData {
    let mut nav_data = RequestNavData {service_id: 0, numelem: 0, nodeid: -1, fromindexelem: 0 };

    let request_param_str: String = request_data.split("=").skip(2).collect();
    let mut reader = Reader::from_str(request_param_str.as_str());
    reader.trim_text(true);

    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"serviceid" => {
                        match reader.read_event(&mut buf) {
                            Ok(Event::Text(e)) => {
                                nav_data.service_id = e.unescape_and_decode(&reader).unwrap().parse().unwrap();
                            },
                            _ => panic!("Expected text in serviceid at position {}", reader.buffer_position()),
                        }
                    },
                    b"numelem" => {
                        match reader.read_event(&mut buf) {
                            Ok(Event::Text(e)) => {
                                nav_data.numelem = e.unescape_and_decode(&reader).unwrap().parse().unwrap();
                            },
                            _ => panic!("Expected text in numelement at position {}", reader.buffer_position()),
                        }
                    },
                    b"nodeid" => {
                        match reader.read_event(&mut buf) {
                            Ok(Event::Text(e)) => {
                                nav_data.nodeid = e.unescape_and_decode(&reader).unwrap().parse().unwrap();
                            },
                            _ => panic!("Expected text in serviceid at position {}", reader.buffer_position()),
                        }
                    },
                    b"fromindexelem" => {
                        match reader.read_event(&mut buf) {
                            Ok(Event::Text(e)) => {
                                nav_data.fromindexelem = e.unescape_and_decode(&reader).unwrap().parse().unwrap();
                            },
                            _ => panic!("Expected text in numelement at position {}", reader.buffer_position()),
                        }
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
    nav_data
}

fn forge_xml_element(name: &str, content: String) -> XMLElement {
    let mut element = XMLElement::new(name);
    element.set_text(content);
    return element;
}

fn forge_optional_xml_element(name: &str, content: Option<&String>) -> XMLElement {
    if content.is_some() {
        return forge_xml_element(name, content.unwrap().to_string());
    }
    return XMLElement::new(name);
}

fn nodes_to_xml(nodes: &Vec<Node>) -> XMLElement {
    let mut root = XMLElement::new("contentdataset");
    for node in nodes {
        match node.node_type {
            Nodetypes::Stream => root.add_element(stream_to_xml(&node)),
            Nodetypes::File => root.add_element(file_to_xml(&node)),
            _ => root.add_element(node_to_xml(&node))
        }
    }

    return root;
}

fn node_to_xml(node: &Node) -> XMLElement {
    let mut root = XMLElement::new("contentdata");

    root.add_element(forge_xml_element("name", node.title.to_string()));
    root.add_element(forge_xml_element("nodeid", node.id.to_string()));

    //root.add_element(XMLElement::new("alphanumeric"));
    root.add_element(XMLElement::new("branch"));

    return root;
}

fn stream_to_xml(node: &Node) -> XMLElement {
    let mut root: XMLElement = XMLElement::new("contentdata");

    root.add_element(forge_xml_element("name", node.title.to_string()));
    root.add_element(forge_xml_element("title", node.title.to_string()));
    // TODO: URL prefix
    root.add_element(forge_optional_xml_element("url", node.url.as_ref()));
    root.add_element(forge_xml_element("nodeid", node.id.to_string()));
    root.add_element(XMLElement::new("playable"));

    return root;
}

fn file_to_xml(node: &Node) -> XMLElement {
    let mut root: XMLElement = XMLElement::new("contentdata");

    root.add_element(forge_optional_xml_element("album", node.album.as_ref()));
    root.add_element(forge_xml_element("genre", "".to_string()));
    root.add_element(forge_xml_element("name", node.title.to_string()));
    root.add_element(forge_xml_element("playlength", "100".to_string()));
    root.add_element(forge_xml_element("title", node.title.to_string()));
    // TODO: URL prefix
    root.add_element(forge_optional_xml_element("url", node.url.as_ref()));
    root.add_element(forge_optional_xml_element("artist", node.artist.as_ref()));
    root.add_element(forge_xml_element("nodeid", node.id.to_string()));
    root.add_element(XMLElement::new("playable"));

    return root;
}
