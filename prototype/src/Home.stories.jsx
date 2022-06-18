import React from "react";
import "../../static/bootstrap-5.2.0-beta1.min.css";
import "../../static/fontawesome-6.1.1/css/all.min.css";

const Home = () => {
  return (
    <>
      <style>
        {`
        html, body, #root { width: 100%; height: 100%!important; }

        .tags-container > * {
          margin-left: 0.125rem;
          margin-top: 0.125rem;
        }

        .editor {
          width: 20rem;
          min-width: 20rem;
          padding: 0.25rem 1rem;
          border-left: 1px solid #ccc;
        }
        `}
      </style>

      <div
        className="d-flex flex-column"
        style={{ width: "100%", height: "100%" }}
      >
        <nav
          className="navbar navbar-dark navbar-expand flex-shrink-1"
          style={{ backgroundColor: "#0d503c" }}
        >
          <div className="container-fluid">
            <a href="#" className="navbar-brand">
              Lifelogger
            </a>
            <ul className="navbar-nav me-auto">
              <li className="nav-item">
                <a href="#" className="nav-link active">
                  Home
                </a>
              </li>
              <li className="nav-item">
                <a href="#" className="nav-link">
                  Topics
                </a>
              </li>
              <li className="nav-item">
                <a href="#" className="nav-link">
                  Settings
                </a>
              </li>
            </ul>
          </div>
        </nav>

        <div className="d-flex flex-grow-1" style={{ minHeight: "0" }}>
          <div className="flex-grow-1" style={{ overflowY: "auto" }}>
            <div id="body" className="container" style={{ marginTop: "1em" }}>
              <blockquote className="blockquote">
                Welcome, Southball!
              </blockquote>

              <div className="row row-cols-1 row-cols-md-2 gx-4">
                <div className="col">
                  <p className="h3">Todos</p>
                  <ul className="list-group">
                    {new Array(10).fill(null).map((_, todoIndex) => (
                      <li className="list-group-item">
                        <div>
                          <b>Todo {todoIndex}</b>
                          <span className="ms-2">
                            <i className="fa-solid fa-calendar-days" />
                            &nbsp;2022/06/07 23:59
                          </span>
                        </div>
                        <div className="d-flex tags-container flex-wrap">
                          {new Array(12).fill(null).map((_, topicIndex) => (
                            <span
                              className="badge bg-success"
                              style={{ fontSize: "0.6rem" }}
                            >
                              Topic {(topicIndex + 1).toString()}
                            </span>
                          ))}
                        </div>
                        <p className="mb-0">Todo Description {todoIndex}</p>
                      </li>
                    ))}
                  </ul>
                </div>
                <div className="col">
                  <p className="h3">Events</p>
                  <ul className="list-group">
                    {new Array(10).fill(null).map((_, eventIndex) => (
                      <li className="list-group-item">
                        <div>
                          <b>Event {eventIndex}</b>
                          <span className="ms-2 me-1">
                            <i className="fa-solid fa-calendar-days" />
                            &nbsp;2022/06/07 00:00 ~ 2022/06/09 23:59
                          </span>
                        </div>
                        <div className="d-flex tags-container flex-wrap">
                          {new Array(12).fill(null).map((_, topicIndex) => (
                            <span
                              className="badge bg-success"
                              style={{ fontSize: "0.6rem" }}
                            >
                              Topic {topicIndex}
                            </span>
                          ))}
                        </div>
                        <p className="mb-0">Todo Description {eventIndex}</p>
                      </li>
                    ))}
                  </ul>
                </div>
              </div>
            </div>
          </div>
          <div id="editor" className="editor" style={{ overflowY: "auto" }}>
            <p className="h4">Edit Todo</p>

            <form>
              <div className="mb-2">
                <label
                  className="form-label mb-0"
                  style={{ fontSize: "0.875rem" }}
                >
                  Title
                </label>
                <input
                  className="form-control form-control-sm"
                  type="text"
                  value="Todo 1"
                />
              </div>

              <div className="mb-2">
                <label
                  className="form-label mb-0"
                  style={{ fontSize: "0.875rem" }}
                >
                  Description
                </label>
                <textarea className="form-control form-control-sm">
                  Todo 1 Description
                </textarea>
              </div>

              <div className="mb-2">
                <label
                  className="form-label mb-0"
                  style={{ fontSize: "0.875rem" }}
                >
                  Topic
                </label>
                <select className="form-select form-select-sm" value="topic-1">
                  <option value="topic-1">Topic 1</option>
                  <option value="topic-2">Topic 2</option>
                </select>
              </div>
            </form>
          </div>
        </div>
      </div>
    </>
  );
};

export default {
  title: "Home",
  component: Home,
};

export const Primary = () => <Home />;
