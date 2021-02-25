import React from "react";
import { Nav } from "react-bootstrap";
import Navbar from "react-bootstrap/Navbar";


export const CustomNavbar = () => {
    return (
        <Navbar bg="light" expand="lg">
            <Navbar.Brand href="#home">Kerf Adjuster</Navbar.Brand>
            <Nav className="mr-auto">
                <Nav.Link href="https://github.com/ritikmishra/kerf-adjuster">GitHub repo</Nav.Link>
            </Nav>
            <Navbar.Toggle aria-controls="basic-navbar-nav"/>
        </Navbar>
    );
};