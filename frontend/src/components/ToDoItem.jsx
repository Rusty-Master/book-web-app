import React, { Component } from 'react';
import axios from "axios";
import "../App.css";

class ToDoItem extends Component {
    state = {
        "title": this.props.title,
        "status": this.props.status,
        "button": this.processStatus(this.props.status),
    }

    // convert status into button label
    processStatus(status) {
        if (status === "Pending") {
            return "edit"
        } else {
            return "delete"
        }
    }

    // modifies status
    inverseStatus(status) {
        if (status === "Pending") {
            return "Done"
        } else {
            return "Pending"
        }
    }

    sendRequest = () => {
        axios.post("http://127.0.0.1:8080/v1/item/" + this.state.button,
            {
                "title": this.state.title,
                "status": this.inverseStatus(this.state.status)
            },
            { headers: { "token": localStorage.getItem("user-token") } })
            .then(response => {
                this.props.passBackResponse(response)
            }).catch(error => {
                this.props.logout();
            });
    }

    // send edit/delete request
    render() {
        return (
            <div className="itemContainer">
                <p>{this.state.title}</p>
                <div className="actionButton" onClick=
                    {this.sendRequest}>
                    {this.state.button}</div>
            </div>
        )
    }
}

export default ToDoItem;