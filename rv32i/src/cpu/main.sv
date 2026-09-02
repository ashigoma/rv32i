module main;
  logic clk;
  logic rst;

  string exec_file;
  string trace_file;
  string log_file;
  string vcd_file;

  int trace_fd;
  int log_fd;

  initial begin
    clk = 1'b0;
    forever #5 clk = ~clk;
  end

  initial begin
    if (!$value$plusargs("EXEC=%s", exec_file)) exec_file = "program.bin";
    if (!$value$plusargs("TRACE_FILE=%s", trace_file)) trace_file = "trace.log";
    if (!$value$plusargs("LOG_FILE=%s", log_file)) log_file = "stdout.log";
    if (!$value$plusargs("VCD_FILE=%s", vcd_file)) vcd_file = "rv32i.vcd";

    trace_fd = $fopen(trace_file, "w");
    log_fd = $fopen(log_file, "w");

    rst = 1'b1;
    #20;
    rst = 1'b0;

    #100000;
    $fclose(trace_fd);
    $fclose(log_fd);

    $dumpfile(vcd_file);
    $dumpvars(0, rv32i_);

    $finish;
  end

  rv32i rv32i_ (
      .clk     (clk),
      .rst     (rst),
      .trace_fd(trace_fd),
      .log_fd  (log_fd)
  );

endmodule
