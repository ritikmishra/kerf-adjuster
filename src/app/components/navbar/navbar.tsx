import React from "react";
import Navbar from "react-bootstrap/Navbar";


export const CustomNavbar = () => {
    return (
        <Navbar bg="light" expand="lg">
            <Navbar.Brand href="#home">Kerf Adjuster</Navbar.Brand>
            <Navbar.Toggle aria-controls="basic-navbar-nav"/>
        </Navbar>
    );
};