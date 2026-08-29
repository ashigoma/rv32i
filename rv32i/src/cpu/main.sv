module main;
    logic ck;
    logic rstn;

    string exec_file;
    string trace_file;
    string log_file;

    int trace_fd;
    int log_fd;

    initial begin
        ck = 1'b0;
        forever #5 ck = ~ck;
    end

    initial begin
        if (!$value$plusargs("EXEC=%s", exec_file))       exec_file = "program.bin";
        if (!$value$plusargs("TRACE_FILE=%s", trace_file)) trace_file = "trace.log";
        if (!$value$plusargs("LOG_FILE=%s", log_file))     log_file = "stdout.log";

        trace_fd = $fopen(trace_file, "w");
        log_fd   = $fopen(log_file, "w");

        rstn = 1'b0;
        #20;
        rstn = 1'b1;

        #200;
        $fclose(trace_fd);
        $fclose(log_fd);
        $finish;
    end

    rv32i u_rv32i (
        .ck        (ck),
        .rstn      (rstn),
        .trace_fd  (trace_fd),
        .log_fd    (log_fd)
    );

endmodule